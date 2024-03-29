# Contributing

## Where to start

All contributions, bug reports, bug fixes, documentation improvements, enhancements, and ideas are welcome.

The best place to start is to check the [issues](https://github.com/rust-practice/cargo-leet)
for something that interests you.
There are also other options in [discussions](https://github.com/rust-practice/cargo-leet/discussions), so feel free to pick from there as well.

## Bug Reports

Please see the [issue templates](https://github.com/rust-practice/cargo-leet/issues/new/choose) that describe the types of information that we are looking for but no worries just fill it the best you can and we'll go from there.

## Working on the code

### Fork the project

In order to work on the project you will need your own fork. To do this click the "Fork" button on
this project.

Once the project is forked you can work on it directly in github codespaces without needing to install anything by clicking the green button near the top and switching to code spaces.
According to [github docs](https://docs.github.com/en/codespaces/overview#billing-for-codespaces) by default you can only use it up to the free amount so you don't need to worry about charges.
Feel free to check some [notes collected](https://c-git.github.io/github/codespaces/) on how to use codespaces with rust (You don't need trunk for this project).

Alternatively you can clone it to your local machine. The following commands creates the directory cargo-leet and connects your repository to the upstream (main project) repository.

```sh
git clone https://github.com/your-user-name/cargo-leet.git
cd cargo-leet
git remote add upstream git@github.com:rust-practice/cargo-leet.git
```

### Creating a branch

You want your main branch to reflect only production-ready code.
Please base your branch on the develop branch which is the default in cargo-leet repo so create a feature branch for
making your changes.
For example:

```sh
git checkout -b my-new-feature
```

This changes your working directory to the my-new-feature branch. Keep any changes in this branch
specific to one bug or feature so the purpose is clear. You can have many my-new-features and switch
in between them using the git checkout command.

When creating this branch, make sure your develop branch is up to date with the latest upstream
develop version. To update your local develop branch, you can do:

```sh
git checkout develop
git pull upstream develop --ff-only
```

### Code linting, formatting, and tests

You can run linting on your code at any time with:

```sh
cargo clippy
```

To format the code run:

```sh
cargo fmt
```

To run the tests:

Note the following is overridden in `.cargo/config.toml` to run with all features enabled

```sh
cargo t
```

Please run these checks before submitting your pull request.

## Committing your code

Once you have made changes to the code on your branch you can see which files have changed by running:

```sh
git status
```

If new files were created that and are not tracked by git they can be added by running:

```sh
git add .
```

Now you can commit your changes in your local repository:

```sh
git commit -am 'Some short helpful message to describe your changes'
```

## Push your changes

Once your changes are ready and all linting/tests are passing you can push your changes to your forked repository:

```sh
git push origin my-new-feature
```

origin is the default name of your remote repository on GitHub. You can see all of your remote repositories by running:

```sh
git remote -v
```

## Making a Pull Request

After pushing your code to origin it is now on GitHub but not yet part of the cargo-leet project.
When you’re ready to ask for a code review, file a pull request. Before you do, once again make sure
that you have followed all the guidelines outlined in this document regarding code style, tests, and
documentation. You should also double check your branch changes against the branch it was based on by:

1. Navigating to your repository on GitHub
1. Click on Branches
1. Click on the Compare button for your feature branch
1. Select the base and compare branches, if necessary. This will be develop and my-new-feature, respectively.

### Make the pull request

If everything looks good, you are ready to make a pull request. This is how you let the maintainers
of the cargo-leet project know you have code ready to be reviewed. To submit the pull request:

1. Navigate to your repository on GitHub
1. Click on the Pull Request button for your feature branch
1. You can then click on Commits and Files Changed to make sure everything looks okay one last time
1. Write a description of your changes in the Conversation tab
1. Click Send Pull Request

This request then goes to the repository maintainers, and they will review the code.

### Updating your pull request

Changes to your code may be needed based on the review of your pull request.
If this is the case you can make them in your branch, add a new commit to that branch, push it to GitHub, and the pull request will be automatically updated.
Pushing them to GitHub again is done by:

```sh
git push origin my-new-feature
```

This will automatically update your pull request with the latest code and restart the Continuous Integration tests.

Another reason you might need to update your pull request is to solve conflicts with changes that have been merged into the develop branch since you opened your pull request.

To do this, you need to rebase your branch:

```sh
git checkout my-new-feature
git fetch upstream
git rebase upstream/develop
```

There may be some merge conflicts that need to be resolved.
After the feature branch has been update locally, you can now update your pull request by pushing to the branch on GitHub:

```sh
git push origin my-new-feature
```

If you rebased and get an error when pushing your changes you can resolve it with:

```sh
git push origin my-new-feature --force
```

## Delete your merged branch (optional)

Once your feature branch is accepted into upstream, you’ll probably want to get rid of the branch.
First, merge upstream develop into your develop branch so git knows it is safe to delete your branch:

```sh
git fetch upstream
git checkout develop
git merge upstream/develop
```

Then you can do:

```sh
git branch -d my-new-feature
```

Make sure you use a lower-case -d, or else git won’t warn you if your feature branch has not actually been merged.

The branch will still exist on GitHub, so to delete it there do:

```sh
git push origin --delete my-new-feature
```
