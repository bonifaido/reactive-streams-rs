use std::error::Error;

pub trait Subscription {
    fn cancel(&self);
    fn request(&self, n: usize);
}

pub trait Subscriber<T> {
    fn on_complete(&self);
    fn on_error<E: Error>(&self, e: E);
    fn on_next(&self, t: T);
    fn on_subscribe<S: Subscription>(&self, s: S);
}

pub trait Publisher<T> {
    fn subscribe<S: Subscriber<T>>(&self, s: S);
}

pub trait Processor<T, R>: Subscriber<T> + Publisher<R> {
}

#[cfg(test)]
mod tests {
    use ::{Publisher, Subscriber, Subscription};
    use std::error::Error;

    struct MySubscriber;
    impl Subscriber<usize> for MySubscriber {
        fn on_complete(&self) {

        }
        fn on_error<E: Error>(&self, e: E) {

        }
        fn on_next(&self, t: usize) {

        }
        fn on_subscribe<S: Subscription>(&self, s: S) {

        }
    }

    struct MyPublisher;
    impl Publisher<usize> for MyPublisher {
        fn subscribe<S: Subscriber<usize>>(&self, s: S) {

        }
    }

    #[test]
    fn it_works() {
    }
}
