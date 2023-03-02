#echo "$1"
function write_fan_recalc()
{
	pwm1_enable=$1
	pwm1_path=$(sudo find /sys/devices/platform/oxp-platform/hwmon/ -name pwm1_enable | sed -n '1p')
	sudo echo "$pwm1_enable" > "$pwm1_path"
	sudo echo "pwm1_enable=$pwm1_enable" >> /tmp/fantastic_sh_tool.log
}

function write_fan_target()
{
	pwm1=$1
	pwm1_path=$(sudo find /sys/devices/platform/oxp-platform/hwmon/ -name pwm1 | sed -n '1p')
	sudo echo "$pwm1" > "$pwm1_path"
	sudo echo "pwm1=$pwm1" >> /tmp/fantastic_sh_tool.log
}

if [ -n "$1" ]; then
	case "$1" in
		--read_thermal_zone)			echo "$(cat /sys/class/drm/card0/device/hwmon/hwmon*/temp1_input)";;
		--read_fan)						echo "$(cat /sys/devices/platform/oxp-platform/hwmon/hwmon*/fan1_input)";;
		--write_fan_recalc)				write_fan_recalc $2;;
		--write_fan_target)				write_fan_target $2;;
	esac
fi
