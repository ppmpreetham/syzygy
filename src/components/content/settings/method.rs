pub enum method {
    Dropdown(Vec<String>),
    /// min, max
    Slider {
        min: f32,
        max: f32,
    },
    Input,
    Radio(Vec<String>),
}
