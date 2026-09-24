busctl --user call com.ddcutil.DdcutilService /com/ddcutil/DdcutilObject com.ddcutil.DdcutilInterface SetVcp isyqu 1 "" 0x10  90  0
busctl --user call com.ddcutil.DdcutilService /com/ddcutil/DdcutilObject com.ddcutil.DdcutilInterface SetVcpWithContext isyqsu 1 "" 0x10  60 "my_app" 0
busctl --user call com.ddcutil.DdcutilService /com/ddcutil/DdcutilObject com.ddcutil.DdcutilInterface GetVcp isyu 1 "" 0x10 0
busctl --user call com.ddcutil.DdcutilService /com/ddcutil/DdcutilObject com.ddcutil.DdcutilInterface GetMultipleVcp isayu 1 "" 2 0x10 0x12 0
busctl --user call com.ddcutil.DdcutilService /com/ddcutil/DdcutilObject com.ddcutil.DdcutilInterface GetVcpMetadata isyu 1 "" 0x10 0
busctl --user call com.ddcutil.DdcutilService /com/ddcutil/DdcutilObject com.ddcutil.DdcutilInterface GetCapabilitiesString isu 1 ""  0
busctl --user call com.ddcutil.DdcutilService /com/ddcutil/DdcutilObject com.ddcutil.DdcutilInterface GetCapabilitiesMetadata isu 1 ""  0