echo "Enter Commit msg: "
read  commit
# _commit = Init Rust Repo

git add .
git commit -m "$commit"
git push 
