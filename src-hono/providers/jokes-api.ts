import { JokeProvider, Joke } from './types';

export class JokesApiProvider implements JokeProvider {
  name = 'JokesAPI (jokeapi.dev)';
  baseUrl = 'https://v2.jokeapi.dev';
  
  private categories = [
    'any', 'miscellaneous', 'programming', 'dark', 'pun', 'spooky', 'christmas'
  ];

  async getRandomJoke(): Promise<Joke> {
    const response = await fetch(`${this.baseUrl}/joke/Any?safe-mode`);
    const data = await response.json();
    
    return this.normalizeJoke(data);
  }

  async getJokeByCategory(category: string): Promise<Joke> {
    const validCategory = this.categories.includes(category.toLowerCase()) ? category : 'Any';
    const response = await fetch(`${this.baseUrl}/joke/${validCategory}?safe-mode`);
    const data = await response.json();
    
    return this.normalizeJoke(data);
  }

  getSupportedCategories(): string[] {
    return this.categories;
  }

  private normalizeJoke(data: any): Joke {
    if (data.type === 'single') {
      return {
        id: data.id?.toString(),
        joke: {
          content: data.joke
        },
        category: data.category?.toLowerCase(),
        type: 'single',
        safe: data.safe,
        lang: data.lang
      };
    } else {
      return {
        id: data.id?.toString(),
        joke: {
          setup: data.setup,
          punchline: data.delivery
        },
        category: data.category?.toLowerCase(),
        type: 'twopart',
        safe: data.safe,
        lang: data.lang
      };
    }
  }
}
