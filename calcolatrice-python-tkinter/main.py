import tkinter as tk

class Calculator:
  def __init__(self, master):
    # Crea la finestra della calcolatrice
    self.master = master
    master.title("Calcolatrice")

    # Crea gli elementi della finestra
    self.display = tk.Entry(master, width=35, bg="lightgrey")
    self.display.grid(row=0, column=0, columnspan=5)

    # Crea i pulsanti numerici
    self.button_1 = tk.Button(master, text="1", width=5, command=lambda: self.add_to_display("1"))
    self.button_2 = tk.Button(master, text="2", width=5, command=lambda: self.add_to_display("2"))
    self.button_3 = tk.Button(master, text="3", width=5, command=lambda: self.add_to_display("3"))
    self.button_4 = tk.Button(master, text="4", width=5, command=lambda: self.add_to_display("4"))
    self.button_5 = tk.Button(master, text="5", width=5, command=lambda: self.add_to_display("5"))
    self.button_6 = tk.Button(master, text="6", width=5, command=lambda: self.add_to_display("6"))
    self.button_7 = tk.Button(master, text="7", width=5, command=lambda: self.add_to_display("7"))
    self.button_8 = tk.Button(master, text="8", width=5, command=lambda: self.add_to_display("8"))
    self.button_9 = tk.Button(master, text="9", width=5, command=lambda: self.add_to_display("9"))
    self.button_0 = tk.Button(master, text="0", width=5, command=lambda: self.add_to_display("0"))
    
    # Crea i pulsanti di operazione
    self.button_add = tk.Button(master, text="+", width=5, command=lambda: self.add_to_display("+"))
    self.button_sub = tk.Button(master, text="-", width=5, command=lambda: self.add_to_display("-"))
   
  tk.mainloop
