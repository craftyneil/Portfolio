from conversion import *
from colors import *
from verification import *
import sys, os, platform
from typing import Union
from random import randint



def clear():
    if platform.system() == "Windows":
        os.system("cls")
    else:
        os.system("clear")

clear()

while True:    
    need_color = input("Etes-vous dans le terminal ? (pour les couleurs) [O/n] : ").upper()
    if need_color in ["N", "NON"]:
        bcolors.disable(bcolors)
        break
    elif need_color in ["", "O", "OUI"]:
        break
    else:
        print("Ceci n'est pas une réponse correcte !")    

HEX_COL = bcolors.HEADER
DEC_COL = bcolors.OKBLUE
BIN_COL = bcolors.OKCYAN
ENDC = bcolors.ENDC
WARN = bcolors.WARNING
FAIL = bcolors.FAIL
OK = bcolors.OKGREEN

clear()

def get_list():
    '''
    affiche la liste des choix de convetions
    '''
    print(f"+----------------------------+-----------------------------------------------------------+-------------------+")
    print(f"|   veuillez choisir votre   |                  {WARN}Aucune lettre ne doit étre{ENDC}               |         0         |")
    print(f"|    option de conversion    |                    {WARN}utilisé pour choisir !{ENDC}                 |         |         |")
    print(f"+----------------------------+-----------------------------+-----------------------------+         V         |")
    print(f"| 1-{HEX_COL}Héxadécimal vers décimal{ENDC} | 2- {DEC_COL}Décimal vers Binaire{ENDC}     | 3- {DEC_COL}Décimal vers Héxadécimal{ENDC} |                   |")
    print(f"+----------------------------+-----------------------------+-----------------------------+     Arréter la    |")
    print(f"| 4- {BIN_COL}Binaire vers décimal{ENDC}    | 5- {HEX_COL}HéxaDécimal vers Binaire{ENDC} | 6- {BIN_COL}Binaire vers Héxadécimal{ENDC} |     conversion    |")
    print(f"+----------------------------+-----------------------------+-----------------------------+-------------------+")
    
def get_choice():
    '''
    récupère le choix de l'utilisateur
    '''
    allfunctions = ["hexadecimale_vers_decimale", "decimale_vers_binaire", "decimale_vers_hexadecimale",
                                "binaire_vers_decimale", "hexadecimale_vers_binaire", "binaire_vers_hexadecimale"]
    get_list()
    while True:
        print("|")
        choix = handle_invalid_choice_error(input("+- Veuillez choisir votre option de conversion : "))
        if choix == -1: continue
          # python considère le 0 comme False
        elif int(choix) == 0: 
            clear()
            sys.exit() 
        else:
            function_name = allfunctions[int(choix) - 1]
            return convert_with(function_name)
        
def convert_with(func):
    '''
    converti le nombre choisi par l'utilisateure grace à la fonction demander
    '''
    while True:
        print("|")
        number = input("+- Entrer le nombre à convertir : ").upper()
        if verify_input(func, number):
            number = verify_input(func, number)
            break

    result = globals()[func](number)
    if func.endswith("hexadecimale"): return " ".join(liste(str(result))) 
    elif func.endswith("decimale"): return " ".join(chunk_string_from_right(str(result), 3))
    elif func.endswith("binaire"): return " ".join(chunk_string_from_right(str(result), 4))

print("+----------------------------+")
print("|           Bonjour          |")
print("+----------------------------+")

while True:
    reponse = get_choice()
    print("|")
    print(f"+-------> {OK}{reponse}{ENDC}")
    print("|")
    print("|")
    while True:    
        _continue = input("Voulez-vous continuer à convertir ? [O/n] : ").upper()
        if _continue in ["N", "NON"]:
            clear()
            sys.exit()
        elif _continue in ["", "O", "OUI"]:
            clear()
            break
        else:
            print(f"{FAIL}Ceci n'est pas une réponse correcte !{ENDC}")   