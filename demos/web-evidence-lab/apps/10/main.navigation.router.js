// consumes: main.navigation.route URL fragment
                export function mountRouter() {
                  const route = () => { const about = location.hash === '#about';
                    document.querySelector('#greeting-panel').hidden = about;
                    document.querySelector('#about-panel').hidden = !about; };
                  window.addEventListener('hashchange', route); route();
                }
