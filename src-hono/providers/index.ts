export * from './types';
export { JokesApiProvider } from './jokes-api';
export { DadJokesProvider } from './dad-jokes';
export { ChuckNorrisProvider } from './chuck-norris';
export { OfficialJokeProvider } from './official-joke';
export { Sv443JokeProvider } from './sv443-joke';
export { JokesOneProvider } from './jokes-one';

import { JokesApiProvider } from './jokes-api';
import { DadJokesProvider } from './dad-jokes';
import { ChuckNorrisProvider } from './chuck-norris';
import { OfficialJokeProvider } from './official-joke';
import { Sv443JokeProvider } from './sv443-joke';
import { JokesOneProvider } from './jokes-one';
import { JokeProvider } from './types';

export const ALL_PROVIDERS: JokeProvider[] = [
  new JokesApiProvider(),
  new DadJokesProvider(),
  new ChuckNorrisProvider(),
  new OfficialJokeProvider(),
  new Sv443JokeProvider(),
  new JokesOneProvider()
];
