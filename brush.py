import lib.M_frac as M_frac
import lib.M_menu as M_menu

while(True):
    M_menu.show_subjec_menu()
    message = M_frac.get_message()
    M_frac.react(message)


