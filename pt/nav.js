class NavigationBar extends HTMLElement {
    constructor() {
        super();
        // O Shadow DOM isola o estilo do componente do resto do site
        this.attachShadow({ mode: 'open' });
    }

    // Método chamado automaticamente quando o elemento é inserido na tela
    connectedCallback() {
        this.render();
        this.setupEventListeners();
    }

    render() {
        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    /* Variáveis locais do componente */
                    --nav-bg: rgba(20, 20, 20, 0.6);
                    --nav-border: rgba(255, 255, 255, 0.1);
                    --text-main: #ffffff;
                    --text-muted: #888888;
                }

                nav {
                    position: fixed;
                    top: 24px;
                    left: 50%;
                    transform: translateX(-50%);
                    display: flex;
                    align-items: center;
                    gap: 32px;
                    padding: 12px 32px;
                    background: var(--nav-bg);
                    backdrop-filter: blur(12px);
                    -webkit-backdrop-filter: blur(12px);
                    border: 1px solid var(--nav-border);
                    border-radius: 50px;
                    z-index: 1000;
                    font-family: 'Inter', sans-serif;
                    box-shadow: 0 4px 30px rgba(0, 0, 0, 0.3);
                }

                .brand {
                    font-weight: 600;
                    color: var(--text-main);
                    font-size: 1rem;
                    text-decoration: none;
                    white-space: nowrap;
                }

                .links {
                    display: flex;
                    gap: 24px;
                }

                .links a {
                    text-decoration: none;
                    color: var(--text-muted);
                    font-size: 0.875rem;
                    font-weight: 400;
                    transition: color 0.3s ease;
                }

                .links a:hover {
                    color: var(--text-main);
                }

                .theme-toggle {
                    background: none;
                    border: none;
                    color: var(--text-muted);
                    cursor: pointer;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    padding: 0;
                    transition: color 0.3s ease, transform 0.2s ease;
                }

                .theme-toggle:hover {
                    color: var(--text-main);
                }
                
                .theme-toggle:active {
                    transform: scale(0.9);
                }

                /* Responsividade embutida no componente */
                @media (max-width: 768px) {
                    .links {
                        display: none; /* Esconde os links no mobile para simplificar */
                    }
                    nav {
                        padding: 12px 24px;
                        gap: 16px;
                    }
                }
            </style>

            <nav>
                <a href="/" class="brand">Ricardo da Rocha</a>
                <div class="links">
                    <a href="/pt/perfil">Sobre</a>
                    <a href="/pt/galeria">Projetos</a>
                    <a href="https://api.whatsapp.com/send?phone=5532984052664&text=Ol%C3%A1+Ricardo%F0%9F%91%8B,+gostaria+de+conversar+sobre+">Contato</a>
                </div>
                <button class="theme-toggle" aria-label="Mudar tema">
                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <circle cx="12" cy="12" r="5"></circle>
                        <line x1="12" y1="1" x2="12" y2="3"></line>
                        <line x1="12" y1="21" x2="12" y2="23"></line>
                        <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
                        <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
                        <line x1="1" y1="12" x2="3" y2="12"></line>
                        <line x1="21" y1="12" x2="23" y2="12"></line>
                        <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
                        <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
                    </svg>
                </button>
            </nav>
        `;
    }

    setupEventListeners() {
        const toggleBtn = this.shadowRoot.querySelector('.theme-toggle');
        toggleBtn.addEventListener('click', () => {
            // Aqui você pode disparar um evento customizado ou alterar o body direto
            console.log("Botão de tema clicado no componente isolado!");
            // document.body.classList.toggle('light-theme');
        });
    }
}

// Registra a nova tag HTML
customElements.define('app-nav', NavigationBar);