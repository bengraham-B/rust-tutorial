echo "Enter Commit msg: "
read  _commit
# _commit = Init Rust Repo

git add .
git commit -m _commit
git push 
