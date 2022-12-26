require 'open-uri'

# Chiedi all'utente di inserire l'URL del sito web da cui scaricare le immagini
puts "Inserisci l'URL del sito web da cui vuoi scaricare le immagini:"
url = gets.chomp

# Apri il sito web e scarica il suo contenuto HTML
html = open(url).read

# Estrai tutti gli URL delle immagini dal contenuto HTML
image_urls = html.scan(/<img.+src="(.+)"/)

# Scarica ogni immagine e salvala nella cartella "immagini"
image_urls.each do |image_url|
  open("immagini/#{File.basename(image_url)}", 'wb') do |file|
    file << open(image_url).read
  end
end

puts "Immagini scaricate con successo!"
