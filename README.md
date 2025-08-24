I’m trying to learn more low-level web development concepts, so I’m building this app in Rust. To make it fun, I came up with the idea of Cat of the Week. It's a web app where each week is split into three phases: an upload window, a voting window, and finally a winner announcement. Basic Auth will implemented.

The repo is a monorepo with three services, all built in Rust:

Backend (Gateway API): handles users, submissions, votes, and the weekly contest logic.

Frontend (Web UI): shows the contest grid, finalists, live voting, and winners.

ML Service (Classifier): an image-processing worker that validates uploads (e.g., “is it a cat?”) and reports results back to the backend.

The project is designed to run locally (I’m thinking of using Kind or Minikube) and serve as a hands-on playground for learning low-level Rust web development, async systems, and service orchestration.
