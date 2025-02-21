mod outermost {
    pub fn middle_function() {}
    fn middle_secret_function() {}
    pub mod inside {
        pub fn inner_function() {
            crate::outermost::middle_secret_function();
        }
        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    // outermost::middle_secret_function(); // cannot access private fn
    outermost::inside::inner_function();
    // outermost::inside::secret_function(); // cannot access private fn
}
