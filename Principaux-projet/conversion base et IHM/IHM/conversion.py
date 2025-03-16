from typing import Union

def divisions2(nombre):
    '''Cette fonction prends comme argument un nombre nombre exprimé en decimal et renvoie une liste
    de 1 et de 0 correspondant aux divisions successives du nombre( avec les restes)
    >>> divisions2(10)
    [0, 1, 0, 1]'''
    resultat = [] # initialisation de la liste de retour qui contiendra les résultats
    # completer les conditions ci dessous
    while (nombre//2 != 0)  and (nombre > 1): # tant qu'il est possible de faire des divisions 

        reste = nombre % 2 # le reste correspond au reste de la division entiere

        nombre = nombre // 2 # la division suivante sera la valeur de la division entiere par 2

        resultat.append(reste) # on ajoute à la liste le reste de la divisionllll

    if (nombre / 2 == 1) or (nombre == 1): # Si on arrive à la derniere division

        resultat.append(nombre % 2) # on ajoute à la liste le dernier reste
    return resultat

def affichage(liste):
    """ cette fonction prend comme argument une liste.
    Elle renvoie une liste dont les éléments ont été inversés
    >>> affichage([0, 0, 1, 1])
    '1100'
    >>> affichage([0, 1, 0, 1])
    '1010'"""
    ordre = [] # initialisation d'une liste vide
    #completer la ligne ci dessous
    for i in range(0, len(liste)):
        ordre.append(str(liste[len(liste)-i-1])) 
    return "".join(ordre)

def equivalence_hexadecimale(valeur):
    """ cette fonction prend pour argument une valeur de type string exprimée en base 16
    et renvoie son équivalent en base 10
    cette fonction est utile à la fonction divisions16
    >>> equivalenceHexa(0)
    '0'
    >>> equivalenceHexa(10)
    'A'"""
    if valeur == 10:
        return "A"
    elif valeur == 11:
        return "B"
    elif valeur == 12:
        return "C"
    elif valeur == 13:
        return "D"
    elif valeur == 14:
        return "E"
    elif valeur == 15:
        return "F"
    else:
        return str(valeur)

def divisions16(nombre):
    """cette fonction prends pour argument un nombre décimal puis à l'aide de divisions successives
    fabrique une liste contenant les restes de division par 16 et le résultat de la derniere division.
    chaque calcul de valeur est au fur et à mesure transcrit en hexadécimal par la fonction equivalenceHexa
    >>> divisions16(1)
    ['1']
    >>> divisions16(689)
    ['1', 'B', '2']"""
    resultat = [] # creation d'une liste vide
    while nombre//16 > 1: # tant que l'on peut encore faire des divisions
        reste = nombre % 16 # on calcule le reste de la division entiére
        nombre = nombre // 16 # on stocke le résultat de la division afin de préparer la prochaine division
        resultat.append(equivalence_hexadecimale(reste)) # on ajoute à la liste le reste de la division converti en hexa
    if nombre//16 <= 15: #s'il n'y a plus qu'une division à faire
        resultat.append(equivalence_hexadecimale(nombre%16)) # on ajoute à la liste le dernier reste
    return resultat #renvoi d'une liste  dans l'ordre inverse, il faudra utiliser la fonction affichage()  

def liste(nombre):
    """ cette fonction prens pour argument une chaine de caratere
        elle renvoie le résultat sous forme d'une liste avec les caracteres separes
        >>> liste("126")
        ['1', '2', '6']
        >>> liste("2F16")
        ['2', 'F', '1', '6']"""
    resultat = [] # initialisation d'une liste vide
    for i in range(0, len(nombre)): # parcours de chacun des caracteres
        resultat.append(nombre[i]) # ajout à la liste du caractere
    return resultat #renvoie de la liste avec comme membre des carateres qu'il faudra convertir en nombre

def equivalence_decimale(nombre):
    """ Cette fonction prends pour argument un caractere et renvoie le nombre equivalent en decimal
    >>> equivalenceDec('1')
    1
    >>> equivalenceDec("A")
    10"""
    if nombre == "A":
        return 10
    elif nombre == "B":
        return 11
    elif nombre == "C":
        return 12
    elif nombre == "D":
        return 13
    elif nombre == "E":
        return 14
    elif nombre == "F":
        return 15
    else:
        return int(nombre)

def hexadecimale_vers_decimale(nombre):
    """ Cette fonction prend pour argument une chaine de caracteres en hexadécimal
    et renvoie la conversion en décimal
    >>> HexDec("8")
    8
    >>> HexDec("2B1")
    689"""
    resultat = 0
    conteneur = liste(str(nombre))
    for i in range(0, len(conteneur)):
        resultat = resultat + equivalence_decimale(conteneur[i])* pow(16,len(conteneur)-1-i)
    return resultat

def decimale_vers_binaire(nombre):
    """ cette fonction prend pour argument un nombre en décimal et renvoie une liste
    correspondand à la valeur binaire de la conversion
    >>> DecBin(10)
    '1010'
    """
    return affichage(divisions2(nombre))

def decimale_vers_hexadecimale(nombre):
    """ Cette fonction prends pour argument une chaine de caracteres de type decimal et renvoie une liste
    correspondant à la valeur hexadecimal de la conversion
    >>> DecHex(689)
    '2B1'
    """
    return affichage(divisions16(int(nombre)))

def binaire_vers_decimale(nombre):
    """ Cette fonction prend pour argument une chaine de caracteres en binaire
    et renvoie la conversion en décimal
    >>> BinDec("1010")
    10
    >>> BinDec("10")
    2
    """
    resultat = 0
    conteneur = liste(nombre)
    for i in range(0, len(conteneur)):
        resultat = resultat + equivalence_decimale(conteneur[i])* pow(2,len(conteneur)-1-i) 
    return resultat

def hexadecimale_vers_binaire(nombre):
    """Cette fonction prends pour argument une chaine de caracteres en Hexadécimal
    et renvoie la conversion en binaire
    >>> HexBin("A")
    '1010'
    """
    resultat = []
    for i in nombre:
        resultat.append(decimale_vers_binaire(equivalence_decimale(i))) 
    return "".join([item for row in resultat for item in row]) # applati la liste

import re
def chunk_string_from_right(string, chunk_size):
    # Découpe une chaîne de caractères en morceaux de longueur constante, mais alignés à droite 
    # "\d{1,chunk_size}" prend un groupe de chiffres entre 1 et chunk_size en partant de la droite avec "\d" et pouvant être suivit par un groupe de longueure chunks_size
    regex = r"\d{1,%d}(?=(?:\d{%d})*$)" %(chunk_size, chunk_size)
    return re.findall(regex, string)

def binaire_vers_hexadecimale(nombre):
    """Cette fonction prends pour argument une chaine de caracteres en binaire
    et renvoie la convertion en Hexadécimal sous forme de liste
    >>> BinHex("1010")
    'A'"""
    resultat = []
    nombre = chunk_string_from_right(nombre, 4)
    for i in nombre:
        resultat.append(equivalence_hexadecimale(binaire_vers_decimale(i)))      
    return affichage(resultat)


if __name__=="__main__":
    import doctest
    doctest.testmod()
