# 📑 Especificação Mestre de Paridade Total: Telegram Desktop ➔ PaperPlane Next

Este documento define o mapeamento exaustivo e integral de todas as capacidades funcionais do **Telegram Desktop Oficial** para implementação direta no **PaperPlane Next**.

---

## 🏗️ 1. Motor & Arquitetura de Dados (TDLib & Sincronização)

| Capacidade | Comportamento Oficial Telegram | Implementação no PaperPlane Next | Status Atual |
| :--- | :--- | :--- | :--- |
| **Carga Inicial de Mensagens** | Carrega de 30 a 50 mensagens em lote único e prioritário ao abrir qualquer chat | Substituir o loop de 2 em 2 (`load_older_messages(2)`) por requisição imediata de 40 mensagens (`load_older_messages(40)`) | 🟡 Pronto para aplicar |
| **Priorização de Chat Ativo** | `openChat` é enviado no instante zero ao clicar na conversa | Mover `perform_chat_action(chat, open_chat)` para o início da seleção de chat | 🟡 Pronto para aplicar |
| **Persistência de Banco Local** | Banco SQLite interno da TDLib indexando mensagens, arquivos e mídias | TDLib já configurado; otimizar `use_message_database: true` e `use_file_database: true` | 🟢 Ativo |
| **Sincronização em Background** | Mantém polling/push ativo da TDLib mesmo com a janela fechada | Janela oculta (`hide`) mantendo o loop GTK e a thread TDLib viva | 🟡 Planejado |

---

## 🖼️ 2. Mídias, Imagens e Visualizador (Media Engine)

| Capacidade | Comportamento Oficial Telegram | Implementação no PaperPlane Next | Status Atual |
| :--- | :--- | :--- | :--- |
| **Clique na Imagem (Visualizador)** | Abre a imagem em visualizador ampliado de alta resolução com fundo escurecido | Criar `MediaViewerWindow` com `AdwWindow` / `GtkPicture` modal, zoom e navegação | 🔴 Faltando |
| **Menu de Contexto de Mídia** | Botão direito: "Salvar Imagem como...", "Copiar Imagem", "Encaminhar", "Excluir" | Adicionar `GtkGestureClick` secundário com menu contextual e `GtkFileDialog` | 🔴 Faltando |
| **Vídeos e Animações** | Reprodução inline com controles, barra de progresso e tela cheia | Integrar `GtkVideo` com controles e atalho para tela cheia | 🟡 Parcial |
| **Áudios e Mensagens de Voz** | Waveform interativo, botão play/pause, velocidades (1x, 1.5x, 2x) | Decodificação de áudio OGG/Opus e componente visual de onda sonora | 🟡 Parcial |
| **Documentos e Downloads** | Exibe ícone, nome, extensão, tamanho e barra circular de progresso | `MessageDocument` com botão de download e ação "Abrir Pasta" | 🟡 Parcial |
| **Stickers e Emojis Animados** | Renderização vetorial fluida a 60 FPS via Lottie / RLottie | `rlottie-sys` + `gtk-rlottie` já integrados | 🟢 Ativo |

---

## 🖥️ 3. Integração com o Desktop Linux (Segundo Plano & Autostart)

| Capacidade | Comportamento Oficial Telegram | Implementação no PaperPlane Next | Status Atual |
| :--- | :--- | :--- | :--- |
| **Fechar para a Bandeja** | Ao clicar no botão 'X' da janela, o app não fecha; oculta a janela e fica na bandeja | Interceptar sinal `close-request` em `Window`, cancelando com `glib::Propagation::Stop` e chamando `window.set_visible(false)` | 🟡 Pronto para aplicar |
| **Ícone no System Tray** | Ícone na barra de tarefas (KDE, GNOME, XFCE) com menu de contexto rápido | Implementação de `StatusNotifierItem` via D-Bus (`org.kde.StatusNotifierItem`) com menu (Abrir, Mensagens Salvas, Silenciar, Sair) | 🟡 Pronto para aplicar |
| **Iniciar com o Sistema** | Opção para iniciar automaticamente junto com o sistema operacional no boot | Criação do arquivo de autostart `~/.config/autostart/app.drey.PaperPlane.desktop` com flag `--hidden` | 🟡 Pronto para aplicar |
| **Notificações do Sistema** | Notificações nativas com avatar de quem enviou, prévia da mensagem e resposta rápida | Conectar ao `notify-send` / Freedesktop Notifications via `ashpd` / D-Bus | 🟡 Parcial |

---

## 🌐 4. Conectividade & Suporte a Proxy

| Capacidade | Comportamento Oficial Telegram | Implementação no PaperPlane Next | Status Atual |
| :--- | :--- | :--- | :--- |
| **MTProto Proxy** | Conexão nativa ao Telegram via proxy MTProto com Host, Porta e Segredo (Secret) | Função `tdlib::functions::add_proxy` com `ProxyType::Mtproto` | 🟡 Planejado |
| **SOCKS5 Proxy** | Suporte a proxy SOCKS5 com autenticação opcional de Usuário e Senha | Função `tdlib::functions::add_proxy` com `ProxyType::Socks5` | 🟡 Planejado |
| **HTTP Proxy** | Suporte a proxy HTTP/HTTPS | Função `tdlib::functions::add_proxy` com `ProxyType::Http` | 🟡 Planejado |
| **Medição de Ping / Latência** | Exibe latência em milissegundos e status (Online / Conectando / Indisponível) | Função `tdlib::functions::ping_proxy` com atualização visual de latência | 🟡 Planejado |
| **Compartilhamento de Links** | Abrir links no formato `tg://proxy?...` e aplicar proxy com 1 clique | Interceptar parâmetros de URL e registrar handler de schema `tg://` | 🟡 Planejado |

---

## 📋 5. Gaveta Lateral Oficial (Drawer Menu)

| Item da Gaveta | Ação no Telegram Oficial | Implementação no PaperPlane Next | Status |
| :--- | :--- | :--- | :--- |
| **Cabeçalho de Usuário** | Avatar circular, Nome Completo, Telefone, `@username` | Card superior com foto, nome e username do usuário autenticado | 🟡 Em andamento |
| **Alternador de Contas** | Dropdown exibindo contas conectadas, contadores e botão "+ Adicionar Conta" | `SidebarSwitcher` / `session.show-sessions` | 🟢 Ativo |
| **Chats Arquivados** | Abre a lista lateral com todas as conversas arquivadas | `session.show-archived` | 🟢 Ativo |
| **Meu Perfil** | Abre janela de edição de foto, nome, bio e nome de usuário | Diálogo de perfil com `tdlib::functions::set_name` e `set_bio` | 🟡 Planejado |
| **Novo Grupo** | Seletor de contatos múltiplos para criação de grupo | Diálogo de contatos com `create_new_basic_group_chat` | 🟡 Planejado |
| **Novo Canal** | Formulário de criação de canal (Título, Descrição, Público/Privado) | Diálogo de criação com `create_new_supergroup_chat` | 🟡 Planejado |
| **Contatos** | Lista telefônica e catálogo de contatos com busca e adição manual | `session.show-contacts` já existente | 🟢 Ativo |
| **Chamadas** | Lista com histórico de chamadas de voz e vídeo | Página de chamadas filtrando mensagens de chamada | 🟡 Planejado |
| **Mensagens Salvas** | Atalho direto que abre a conversa privada do próprio usuário | Ação `session.saved-messages` abrindo `me_().id()` | 🟢 Ativo |
| **Configurações** | Abre a janela principal com todas as categorias | Ação `session.show-preferences` | 🟢 Ativo |
| **Modo Noturno** | Toggle direto para alternar imediatamente entre tema claro e escuro | Ação `session.toggle-dark-mode` com `adw::StyleManager` | 🟢 Ativo |

---

## ⚙️ 6. Janela de Configurações Completa (Preferences)

| Aba | Seções & Controles Mapeados |
| :--- | :--- |
| **Minha Conta** | Foto de perfil, Nome, Sobrenome, Bio, `@username`, Telefone e QR Code |
| **Geral & Interface** | Esquema de cores, Tema escuro, Escala da interface (100% a 150%), Enviar com Enter |
| **Notificações & Sons** | Chats privados, Grupos, Canais, Sons de notificação, Pré-visualização de texto |
| **Privacidade & Segurança** | Sessões ativas (Desconectar outros dispositivos), Verificação em duas etapas (2FA), Bloqueados |
| **Pastas de Chat** | Criar pastas personalizadas, ícones das pastas, incluir/excluir chats |
| **Dados & Armazenamento** | Tamanho do cache, Limpar cache, Limpar banco de dados, Download automático de mídia |
| **Proxy & Conexão** | Usar proxy, Lista de proxies (MTProto, SOCKS5, HTTP), Adicionar proxy, Ping |
| **Idioma** | Seleção de idioma (Português do Brasil como padrão oficial) |
| **Sistema & Inicialização**| Iniciar com o sistema operacional, Minimizar para a barra de tarefas ao fechar |
