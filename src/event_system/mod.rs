pub trait Receiver {
    type Data;
    type Transformer;

    fn on_emit(&self, data: &Self::Data);
}

pub trait Signal {
    type Data;
    type RecType: Receiver;

    fn emit(&self, data: Self::Data);
    fn connect(&mut self, trns: <Self::RecType as Receiver>::Transformer);
    fn disconnect(&mut self, i: usize);
}

macro_rules! signal {
    ($sig:ident<$rectype:ident, $data: ident> = [ $($rec:ident = $cls:expr),* ] ) => {
        pub struct $sig {
            counter: usize,
            recs: Vec<$rectype>
        }

        pub struct $rectype {
            id: usize,
            sig: Box<dyn Fn(&$data)>
        }

        impl $rectype {
            fn new(id: usize, cls: Box<dyn Fn(&$data)>) -> Self {
                Self {
                    id, sig: cls
                }
            }
        }

        impl $sig {
            fn nxt(&mut self) -> usize {
                self.counter += 1;
                self.counter
            }

            const fn new() -> Self {
                $sig {
                    recs: Vec::new(),
                    counter: 0
                }
            }
        }

        impl Receiver for $rectype {
            type Data = $data;
            type Transformer = Box<dyn Fn(&$data)>;

            fn on_emit(&self, data: &Self::Data) {
                (self.sig)(data)
            }
        }

        impl Signal for $sig {
            type Data = $data;
            type RecType = $rectype;

            fn emit(&self, data: Self::Data) {
                self.recs.iter().for_each(|rec| rec.on_emit(&data))
            }

            fn connect(&mut self, trns: <Self::RecType as Receiver>::Transformer) {
                let i: usize = self.nxt();
                let r = Self::RecType::new(i, trns);
                self.recs.push(r);
            }

            fn disconnect(&mut self, i: usize) {
                let idx = self.recs.iter().position(|r| r.id == i).unwrap();
                self.recs.remove(idx);
            }
        }

        static mut SIGNAL: $sig = $sig::new();
    }
}

macro_rules! signal_fns {
    () => {
        fn emit(self) {
            unsafe {
                SIGNAL.emit(self)
            }
        }

        fn listen<F>(callable: F) where F: Fn(&Self) -> () + 'static {
            unsafe {
                SIGNAL.connect(Box::new(callable));
            }
        }
    };
}

pub (crate) use signal;
pub (crate) use signal_fns;