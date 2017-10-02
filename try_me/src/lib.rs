mod outermost {
    pub fn middle_fn() {}

    fn middle_secret_fn() {}

    mod inside {
        pub fn inner_fn() {}

        fn secret_fn() {}
    }
}

fn try_me() {
    outermost::middle_fn();
    outermost::middle_secret_fn();
    outermost::inside::inner_fn();
    outermost::inside::secret_fn();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
