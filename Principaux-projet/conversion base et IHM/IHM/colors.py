class bcolors:
    '''
    cette class permetrera d'ajouter quelque couleurs pour une meilleure lisibilité avec les caractères d'échappements
    '''
    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKCYAN = '\033[96m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'

    @staticmethod
    def disable(self) -> None:
        '''
        desactive les couleurs dans le cas ou l'utilisateur lance le programme autre part que dans le terminal
        '''
        self.HEADER = ""
        self.OKBLUE = ""
        self.OKCYAN = ""
        self.OKGREEN = ""
        self.WARNING = ""
        self.FAIL = ""
        self.ENDC = ""
