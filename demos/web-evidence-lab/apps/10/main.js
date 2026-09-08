import {render} from './main.greeting.view.js';
import {normalize} from './main.greeting.model.js';
import {format} from './main.greeting.locale.js';
import {load, save} from './main.preferences.store.js';
import {mountRouter} from './main.navigation.router.js';
import {request} from './main.greeting.api.client.js';
import {deliver} from './main.greeting.delivery.js';
import {mountHistory} from './main.history.view.js';
const saved = load(); document.querySelector('#name').value = saved.name || 'World'; document.querySelector('#locale').value = saved.locale || 'en';
mountRouter();
const bus = new EventTarget(); mountHistory(bus);
// produces: main.greeting.sent parent publishes accepted greeting
                const form = document.querySelector('#greet-form');
                form.addEventListener('submit', async event => {
                  event.preventDefault(); const error = document.querySelector('#error'); error.textContent = '';
                  form.querySelector('button').disabled = true;
                  try {
                    const name = normalize(document.querySelector('#name').value);
                    const locale = document.querySelector('#locale').value;
                    const result = await deliver(name, locale); const message = result.message; document.querySelector('#transport').textContent = result.source;
                    render(message);
                    save({name, locale});
                    bus.dispatchEvent(new CustomEvent('main.greeting.sent', {detail:message}));
                  } catch (failure) { error.textContent = failure.message; }
                  finally { form.querySelector('button').disabled = false; }
                });
                render(format(document.querySelector('#name').value, document.querySelector('#locale').value));
