MESSAGE=$1
FROM=$1
TO=$2

config:
	cd .git && git remote add github https://github.com/Baabacar/0-shell.git
push:
	git add . && git commit -m "$(MESSAGE)" && git push origin && git push github
r: 
	clear && cargo r
merge:
	git checkout $(TO) && git merge $(FROM) 