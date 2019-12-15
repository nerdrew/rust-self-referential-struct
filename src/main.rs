use std::borrow::Cow;
use std::pin::Pin;

#[derive(Debug)]
struct Inner<'a> {
    buf: String,
    cow: Option<Cow<'a, str>>,
    _pin: std::marker::PhantomPinned,
}

impl<'a> Inner<'a> {
    fn new(buf: String) -> Pin<Box<Self>> {
        let inner = Self {
            buf,
            cow: None,
            _pin: std::marker::PhantomPinned,
        };
        let mut pinned = Box::pin(inner);

        let cow = Cow::Borrowed(pinned.buf.as_str());
        unsafe {
            let cow = std::mem::transmute::<_, Cow<'a, str>>(cow);
            pinned.as_mut().get_unchecked_mut().cow = Some(cow);
        }
        pinned
    }
}

#[allow(dead_code)]
struct Owned<'a> {
    inner: Pin<Box<Inner<'a>>>,
}

#[allow(dead_code)]
impl<'a> Owned<'a> {
    fn buf(&self) -> &str {
        &self.inner.buf
    }

    // here so we can mutate the internals
    fn buf_mut<'b>(&'b mut self) -> &'b mut Inner<'b> {
        unsafe {
            let inner = self.inner.as_mut().get_unchecked_mut();
            std::mem::transmute::<_, &mut Inner<'b>>(inner)
        }
    }

    fn cow<'b>(&'b self) -> &'b Cow<'b, str> {
        unsafe { std::mem::transmute::<&Cow<'a, str>, &Cow<'b, str>>(self.inner.cow.as_ref().unwrap()) }
    }

    fn cow_mut<'b>(&'b mut self) -> &'b mut Cow<'b, str> {
        unsafe {
            let cow = self.inner.as_mut().get_unchecked_mut().cow.as_mut().unwrap();
            std::mem::transmute::<_, &mut Cow<'b, str>>(cow)
        }
    }
}

impl<'a> From<String> for Owned<'a> {
    fn from(buf: String) -> Self {
        Self { inner: Inner::new(buf) }
    }
}

fn main() {
    let mut owned = Owned::from("I see you".to_string());
    let cow = owned.cow().to_owned();

    // owned.buf_mut().buf.get_mut(0..5).unwrap().make_ascii_uppercase();

    dbg!(owned.buf());
    dbg!(&cow);

    drop(owned);

    // dbg!(&cow);
}
