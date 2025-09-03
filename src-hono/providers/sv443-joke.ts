import { JokeProvider, Joke } from './types';

export class Sv443JokeProvider implements JokeProvider {
  name = 'Sv443 JokeAPI';
  baseUrl = 'https://sv443.net/jokeapi/v2';
  
  private categories = [
    'programming', 'miscellaneous', 'dark', 'pun', 'spooky', 'christmas'
  ];

  async getRandomJoke(): Promise<Joke> {
    const response = await fetch(`${this.baseUrl}/joke/Any?safe-mode&type=single,twopart`);
    const data = await response.json();
    
    return this.normalizeJoke(data);
  }

  async getJokeByCategory(category: string): Promise<Joke> {
    const validCategory = this.categories.includes(category.toLowerCase()) ? category : 'Any';
    const response = await fetch(`${this.baseUrl}/joke/${validCategory}?safe-mode&type=single,twopart`);
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
