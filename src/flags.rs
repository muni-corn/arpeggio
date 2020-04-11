#[derive(Default)]
pub struct Flags {
    /// Any image_files passed in
    pub image_files: Vec<String>,

    /// --set, -s: Set the wallpaper to the image given
    pub set: bool,

    /// --force, -f: Force recreation of a palette
    pub force: bool,

    /// --quiet, -q: Print errors only
    pub quiet: bool,

    /// --very-quiet, -qq: Print nothing
    pub very_quiet: bool,

    /// --no-apply, -n: Skip setting wallpaper, terminal colors, etc
    pub no_apply: bool,

    /// --delete, -d: Delete palettes for the arguments given
    pub delete: bool,

    /// --random, -r: Set a random wallpaper
    pub random: bool,

    /// --clock, -c: (coming soon) Treats all images given as a series of wallpapers to change
    /// throughout the day. If given without files, updates the current wallpaper to the correct
    /// time-of-day variant.
    pub clock: bool,
}

pub fn parse() -> Flags {
    // start with default flags
    let mut flags = Flags::default();

    // get args
    let args = std::env::args().skip(1);

    for arg in args {
        match arg.as_str() {
            "--set" | "-s" => flags.set = true,
            "--force" | "-f" => flags.force = true,
            "--quiet" | "-q" => flags.quiet = true,
            "--very-quiet" | "-qq" => flags.very_quiet = true,
            "--no-apply" | "-n" => flags.no_apply = true,
            "--delete" | "-d" => flags.delete = true,
            "--random" | "-r" => flags.random = true,
            "--clock" | "-c" => flags.clock = true,
            _ => flags.image_files.push(arg),
        }
    }

    flags
}
