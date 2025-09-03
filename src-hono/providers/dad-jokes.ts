import { JokeProvider, Joke } from './types';

export class DadJokesProvider implements JokeProvider {
  name = 'icanhazdadjoke';
  baseUrl = 'https://icanhazdadjoke.com';

  async getRandomJoke(): Promise<Joke> {
    const response = await fetch(this.baseUrl, {
      headers: {
        'Accept': 'application/json',
        'User-Agent': 'Jokes App (https://github.com/yourapp)'
      }
    });
    
    const data = await response.json();
    
    return {
      id: data.id,
      joke: {
        content: data.joke
      },
      category: 'dad jokes',
      type: 'single'
    };
  }

  async getJokeByCategory(category: string): Promise<Joke> {
    // Dad jokes API doesn't support categories, so just return a random joke
    return this.getRandomJoke();
  }

  getSupportedCategories(): string[] {
    return ['dad jokes'];
  }
}
