// consumes: main.greeting.sent accepted greeting event
                // produces: main.history.entries rendered recent greetings
                import {append} from './main.history.model.js';
                export function mountHistory(bus) {
                  let items = [];
                  const draw = () => { document.querySelector('#history').replaceChildren(...items.map(text => {
                    const li = document.createElement('li'); li.textContent = text; return li;
                  })); };
                  bus.addEventListener('main.greeting.sent', event => { items = append(items, event.detail); draw(); });
                  document.querySelector('#clear-history').addEventListener('click', () => { items = []; draw(); });
                }
