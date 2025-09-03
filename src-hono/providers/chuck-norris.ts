import { JokeProvider, Joke } from './types';

export class ChuckNorrisProvider implements JokeProvider {
  name = 'Chuck Norris Jokes API';
  baseUrl = 'https://api.chucknorris.io';
  
  private categories = [
    'animal', 'career', 'celebrity', 'dev', 'explicit', 'fashion', 
    'food', 'history', 'money', 'movie', 'music', 'political', 
    'religion', 'science', 'sport', 'travel'
  ];

  async getRandomJoke(): Promise<Joke> {
    const response = await fetch(`${this.baseUrl}/jokes/random`);
    const data = await response.json();
    
    return {
      id: data.id,
      joke: {
        content: data.value
      },
      category: (data.categories?.[0] || 'uncategorized').toLowerCase(),
      type: 'single'
    };
  }

  async getJokeByCategory(category: string): Promise<Joke> {
    const validCategory = this.categories.includes(category.toLowerCase()) 
      ? category.toLowerCase() 
      : null;
    
    if (!validCategory) {
      return this.getRandomJoke();
    }
    
    const response = await fetch(`${this.baseUrl}/jokes/random?category=${validCategory}`);
    const data = await response.json();
    
    return {
      id: data.id,
      joke: {
        content: data.value
      },
      category: (data.categories?.[0] || category).toLowerCase(),
      type: 'single'
    };
  }

  getSupportedCategories(): string[] {
    return this.categories;
  }
}
