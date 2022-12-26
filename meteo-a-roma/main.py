import requests
import tkinter as tk

# Creiamo la finestra principale
root = tk.Tk()
root.title("Meteo a Roma")

# Creiamo una label per visualizzare il meteo
label = tk.Label(root, text="Scaricando le informazioni meteo...")
label.pack()

# Useremo l'API di OpenWeatherMap per ottenere le informazioni
api_key = "YOUR_API_KEY"  # Sostituisci con la tua chiave API
city_id = "4219762"  # ID della città di Roma
url = f"http://api.openweathermap.org/data/2.5/weather?id={city_id}&appid={api_key}"

# Inviamo la richiesta all'API
response = requests.get(url)

# Controlliamo che la richiesta sia andata a buon fine
if response.status_code == 200:
    # Carichiamo il risultato in formato JSON
    data = response.json()

    # Estraiamo le informazioni di interesse
    temperature = data["main"]["temp"] - 273.15  # La temperatura è in Kelvin, la convertiamo in Celsius
    condition = data["weather"][0]["description"]

    # Aggiorniamo il testo della label con le informazioni ottenute
    label["text"] = f"Temperatura: {temperature:.1f}°C\nCondizioni: {condition}"
else:
    # Se la richiesta non è andata a buon fine, mostriamo un messaggio di errore
    label["text"] = "Errore nel recupero delle informazioni meteo"

# Avviamo il ciclo di gestione degli eventi
root.mainloop()
