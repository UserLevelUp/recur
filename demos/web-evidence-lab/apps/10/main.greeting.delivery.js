// consumes: main.greeting.api.response delivery input
                // produces: main.greeting.message delivered or cached greeting
                import {request} from './main.greeting.api.client.js';
                const cache = new Map();
                export async function deliver(name, locale) {
                  const key = JSON.stringify([name, locale]); let failure;
                  for (let attempt = 0; attempt < 2; attempt++) {
                    try { const message = await request(name, locale); cache.set(key, message); return {message, source:'network'}; }
                    catch (error) { failure = error; }
                  }
                  if (cache.has(key)) return {message:cache.get(key), source:'cache'};
                  throw failure;
                }
