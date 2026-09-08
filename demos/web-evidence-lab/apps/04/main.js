import {render} from './main.greeting.view.js';
import {normalize} from './main.greeting.model.js';
const format = name => `Hello, ${name}!`;
// Parent coordinates greeting input and output.
                const form = document.querySelector('#greet-form');
                form.addEventListener('submit', async event => {
                  event.preventDefault(); const error = document.querySelector('#error'); error.textContent = '';
                  form.querySelector('button').disabled = true;
                  try {
                    const name = normalize(document.querySelector('#name').value);
                    const locale = document.querySelector('#locale').value;
                    const message = format(name, locale);
                    render(message);


                  } catch (failure) { error.textContent = failure.message; }
                  finally { form.querySelector('button').disabled = false; }
                });
                render(format(document.querySelector('#name').value, document.querySelector('#locale').value));
document.querySelector('#locale').closest('label').hidden = true;
