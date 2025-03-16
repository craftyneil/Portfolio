
from typing import Union
from colors import bcolors
from random import randint

HEX_COL = bcolors.HEADER
DEC_COL = bcolors.OKBLUE
BIN_COL = bcolors.OKCYAN
ENDC = bcolors.ENDC
WARN = bcolors.WARNING
FAIL = bcolors.FAIL
OK = bcolors.OKGREEN

def is_in_base_n(fonction_name, number, base_num):
    '''
    vérifie si le nombre "number" correspond bien au base "base_num"
    '''
    if str(number) == "": return False # quand l'utilisateur a appuyer sur la barre espace
    base_list = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F"][:base_num]
    for i in str(number):
        if i.upper() not in base_list:
            show_invalid_input_error_message(base_num, base_list, fonction_name, invalid_caracter=i)
            return False
    return number

def verify_input(function_name, number):
    '''
    retourne int (pour le décimale) ou str (pour l'héxadécimale et le binaire) si l'input est valide, sinon False  
    '''
    if function_name.startswith("binaire"): return is_in_base_n(function_name, number, base_num=2)  
    elif function_name.startswith("decimale"): return int(is_in_base_n(function_name, number, base_num=10)) # int() car le nombre est en base 10
    elif function_name.startswith("hexadecimale"): return is_in_base_n(function_name, number, base_num=16)  
    else: return False

def show_invalid_input_error_message(base_num_expected, base_list, fonction_name, invalid_caracter):
    '''
    affiche un message d'erreur pour le cas où l'utilisateur n'a pas écrit un nombre correcte pour la convertion qu'il a choisi avec une solution éventuelle
    '''
    print(f"{FAIL}|-> L'entrée a au moins 1 caractère qui ne se trouvent pas dans la liste suivant {base_list} du base {base_num_expected} de la fonction choisi ({fonction_name}){ENDC}")
    print(f"|-> Remplacer le caractère {FAIL}{invalid_caracter}{ENDC} par au moins 1 caractère dans la liste si-dessus, par exemple {OK}{base_list[randint(1, len(base_list) - 1)]}{ENDC}")

def is_valid_number(number):
    '''
    enlève le signe du nombre pour vérifie si celui-ci est un nombre valide (entier) 
    ''' 
    return (number.startswith("+") or number.startswith("-")) and number[1:].isdigit() or number.isdigit()

def handle_invalid_choice_error(choice):
    '''
    affiche un message d'erreur pour le cas où l'utilisateur n'a pas écrit un nombre correcte pour choisir une fonction de convertion
    '''
    if not is_valid_number(choice):
        print(f"{FAIL}|-> Le choix \"{choice}\"  doit être un entier et non une chaîne de caractères{ENDC}")
        return -1
    elif int(choice) > 6:
        print(f"{FAIL}|->Vous pouvez choisir uniquement entre 6 fonctions, pas {choice}{ENDC}")
        return -1
    elif int(choice) < 0:
        print(f"{FAIL}|-> Un choix négatif n'est pas possible, les seules réponses valides sont entre 0 et 6{ENDC}")
        return -1
    return int(choice)
