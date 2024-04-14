pub const GET_BAT_CAP: &str = "
    cat /sys/class/power_supply/BAT0/capacity
";

pub const GET_BR: &str = "
    brightness=$(cat /sys/class/backlight/intel_backlight/brightness); \
    max_brightness=$(cat /sys/class/backlight/intel_backlight/max_brightness); \
    echo $(($brightness * 100 / $max_brightness)) \
";

pub const FOLLOW_BR: &str = "
    inotifywait -m -q -e modify /sys/class/backlight/intel_backlight/brightness
";

pub const GET_VOL: &str = "
    pamixer --get-volume
";

pub const FOLLOW_VOL: &str = "
    pactl subscribe | grep --line-buffered \"Event 'change' on sink\"
";

pub const FOLLOW_NM_STATE: &str = "
    dbus-monitor --system \"type='signal',interface='org.freedesktop.NetworkManager'\"
";

pub const MPRIS_PREV: &str = "
    playerctl previous
";

pub const MPRIS_TOGGLE: &str = "
    playerctl play-pause
";

pub const MPRIS_NEXT: &str = "
    playerctl next
";

pub const FOLLOW_MPRIS_METADATA: &str = "
    playerctl -F metadata -f '{{xesam:title}}%sep%{{xesam:artist}}%sep%{{mpris:artUrl}}'
";

pub const GET_IMG_GEOMETRY: &str = "identify -format '%w %h' ";

// Used in joined with another string like 1, 2, .., N
pub const JUMP_TO_DESKTOP: &str = "wmctrl -s ";

pub const GET_NUMBER_OF_DESKTOPS: &str = "
    xprop -root _NET_NUMBER_OF_DESKTOPS | cut -d ' ' -f3
";

// Use this when you need a complex desktop widget
// const _GET_CULIENTS_LIST: &str = "
//     wmctrl -l | awk '{print $2}'
// ";
// 
// const _GET_CURRENT_DESKTOP: &str = "
//     wmctrl -d | grep '*' | cut -d ' ' -f1
// ";
// 
// const _FOLLOW_CURRENT_DESKTOP: &str = "
//     xprop -spy -root _NET_CURRENT_DESKTOP | cut -d ' ' -f3
// ";
