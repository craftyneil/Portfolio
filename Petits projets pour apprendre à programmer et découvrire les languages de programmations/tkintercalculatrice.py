
from tkinter import *

formule = "" 

def click(num): 

    global formule 
    formule = formule + str(num) 
    equation.set(formule) 

def equalclick(): 
    try: 
        global formule 

        result = str(eval(formule)) 
        equation.set(result) 
        formule = result

    except: 
        equation.set(" error ") 
        formule = "" 

def effacer(): 
    global formule 
    formule = "" 
    equation.set("") 

if __name__ == "__main__": 
    master = Tk() 
    master.title("Calculatrice") 
    master.geometry("375x315") 
    equation = StringVar() 
    formule_field = Entry(master, textvariable=equation) 
    formule_field.grid(columnspan=4,  pady= 30 , padx = 20 , ipadx = 100 , ipady = 10)
    btn_1 = Button(master, text=' 1 ', command=lambda: click(1), height=2, width=10) 
    btn_1.grid(row=2, column=0) 

    btn_2 = Button(master, text=' 2 ', command=lambda: click(2), height=2, width=10) 
    btn_2.grid(row=2, column=1) 

    btn_3 = Button(master, text=' 3 ', command=lambda: click(3), height=2, width=10) 
    btn_3.grid(row=2, column=2) 

    btn_4 = Button(master, text=' 4 ', command=lambda: click(4), height=2, width=10) 
    btn_4.grid(row=3, column=0) 

    btn_5 = Button(master, text=' 5 ', command=lambda: click(5), height=2, width=10) 
    btn_5.grid(row=3, column=1) 

    btn_6 = Button(master, text=' 6 ', command=lambda: click(6), height=2, width=10) 
    btn_6.grid(row=3, column=2) 

    btn_7 = Button(master, text=' 7 ', command=lambda: click(7), height=2, width=10) 
    btn_7.grid(row=4, column=0) 

    btn_8 = Button(master, text=' 8 ', command=lambda: click(8), height=2, width=10) 
    btn_8.grid(row=4, column=1) 

    btn_9 = Button(master, text=' 9 ', command=lambda: click(9), height=2, width=10) 
    btn_9.grid(row=4, column=2) 

    btn_0 = Button(master, text=' 0 ', command=lambda: click(0), height=2, width=10) 
    btn_0.grid(row=5, column=0) 

    plus = Button(master, text=' + ', command=lambda: click("+"), height=2, width=10) 
    plus.grid(row=2, column=3) 

    minus = Button(master, text=' - ', command=lambda: click("-"), height=2, width=10) 
    minus.grid(row=3, column=3) 

    multiply = Button(master, text=' * ', command=lambda: click("*"), height=2, width=10) 
    multiply.grid(row=4, column=3) 

    divide = Button(master, text=' / ', command=lambda: click("/"), height=2, width=10) 
    divide.grid(row=5, column=3) 

    equal = Button(master, text=' = ', command=equalclick, height=2, width=10) 
    equal.grid(row=5, column=2) 

    effacer = Button(master, text='effacer', command=effacer, height=2, width=10) 
    effacer.grid(row=6, column='0') 

    Decimal= Button(master, text='.', command=lambda: click('.'), height=2, width=10) 
    Decimal.grid(row=5, column=1) 
    
    percent= Button(master, text='%', command=lambda: click('%'), height=2, width=10) 
    percent.grid(row=6, column=1) 
    
    inverse= Button(master, text='1/X',  height=2, width=10) 
    inverse.grid(row=6, column=2) 
    
    memo= Button(master, text='M',  height=2, width=10) 
    memo.grid(row=6, column=3)     
    
    master.mainloop()
