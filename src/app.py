file = open("textures.rs", "r")
content = file.read()

content = content.replace("\n", "")

open("textures.rs", "w").write(content)
