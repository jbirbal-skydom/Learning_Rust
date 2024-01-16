    /// # set_unique_color
    ///
    /// Sets the text color to a unique color using ANSI escape codes with RGB values.
    ///
    /// ## Parameters
    ///
    /// - `r`: Red component (0-255).
    /// - `g`: Green component (0-255).
    /// - `b`: Blue component (0-255).
    ///
    /// ## Example
    ///
    /// ```
    /// use cli_utils::colors_pvt::set_unique_color;
    ///
    /// // Set the text color to a unique color (e.g., RGB: 100, 200, 50)
    /// set_unique_color(100, 200, 50);
    /// ```
    pub fn set_unique_color(r: u8, g: u8, b: u8) {
        let color_code = format!("\x1b[38;2;{};{};{}m", r, g, b);
        print!("{}", color_code);
    }

    /// # reset_color
    ///
    /// Resets the text color to the default using ANSI escape codes.
    ///
    /// ## Example
    ///
    /// ```
    /// use cli_utils::colors_pvt::reset_color;
    ///
    /// // Reset the text color to the default
    /// reset_color();
    /// ```
    pub fn reset_color() {
        let reset_code = "\x1b[0m";
        print!("{}", reset_code);
    }
