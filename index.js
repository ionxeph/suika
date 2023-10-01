import init from './pkg/suika.js';
init().catch((error) => {
  if (!error.message.startsWith('Using exceptions for control flow,')) {
    throw error;
  }
});
