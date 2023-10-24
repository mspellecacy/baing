# BA!ng
This project is an attempt to converge two ideas: 
- The ability to utilize AI to recommend content based on existing personalized collections.
- Creating personalized schedules for Movies and TV (like my other project [Bynger](https://github.com/mspellecacy/bynger/)) from those user defined collections. 

## Getting started
- Prerequisites: 
  - [TMDB API Key](https://developer.themoviedb.org/reference/intro/getting-started)
  - [ChatGPT API Key](https://platform.openai.com/docs/quickstart) (Paid)
- Jumpstart the backend with the [docker compose file](https://github.com/mspellecacy/baing/blob/master/backend/docker-compose.yml)
- Massage the .env file to your needs: [example.env](https://github.com/mspellecacy/baing/blob/master/example.env)
- While in the ``backend/`` run ``sqlx database setup`` to get your DB setup
- While in the project root run ``./run_baing.sh`` to build and run backend & frontend 
- Assuming you've left everything default navigate to (http://localhost:8080/) to create a user.
- Add a TMDB API Key to your profile
- Start discovering new content!

## Want to contribute?
Please feel free to open a ticket, create a fork, or make a pull request.
I'm always open to suggestions and recommendations for new features.

Happy watching!