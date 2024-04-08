pub const GET_BAT_CAP: &str = "
    cat /sys/class/power_supply/BAT0/capacity
";

pub const GET_BR: &str = "
    brightness=$(cat /sys/class/backlight/intel_backlight/brightness); \
    max_brightness=$(cat /sys/class/backlight/intel_backlight/max_brightness); \
    echo $(($brightness * 100 / $max_brightness)) \
";

pub const TRACK_BR: &str = "
    inotifywait -m -q -e modify /sys/class/backlight/intel_backlight/brightness
";

pub const GET_VOL: &str = "
    pamixer --get-volume
";

pub const TRACK_VOL: &str = "
    pactl subscribe | grep --line-buffered \"Event 'change' on sink\"
";

pub const TRACK_NM_STATE: &str = "
    dbus-monitor --system \"type='signal',interface='org.freedesktop.NetworkManager'\"
";
