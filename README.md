# 🏦 Alpha Bank - Backend API (Rust + MySQL)

Esta é a API REST completa do sistema de controle financeiro Alpha Bank, desenvolvida em **Rust** utilizando o framework **Actix-web** e **MySQL**.

## 🚀 Tecnologias Utilizadas

*   **Rust 1.70+**
*   **Actix-web 4.4** - Framework web assíncrono de alto desempenho.
*   **SQLx 0.7** - Driver MySQL com verificação de consultas em tempo de compilação.
*   **MySQL 8.0+** - Banco de dados relacional.
*   **JWT** - Autenticação via JSON Web Tokens.
*   **Bcrypt** - Hash seguro para senhas.

## 🔧 Configuração e Instalação

### 1. Instalação do Rust

Instale o Rust através do `rustup`: https://rustup.rs/

Após a instalação, verifique as versões no terminal:
```bash
rustc --version
cargo --version
```

### 2. Configuração do MySQL

É necessário ter uma instância do MySQL 8.0+ em execução. O MySQL Workbench é recomendado para gerenciamento visual.

**Criação do Banco de Dados:**

Execute o arquivo `schema.sql` para criar o banco de dados `alpha_bank` e todas as tabelas necessárias.

```sql
mysql -u root -p < schema.sql
```

### 3. Configuração de Variáveis de Ambiente

Copie o arquivo de exemplo para criar o arquivo de configuração local:

```bash
cp .env.example .env
```

Edite o arquivo `.env` com suas credenciais e configurações:

```env
HOST=127.0.0.1
PORT=8080

# Ajuste a senha do seu MySQL
DATABASE_URL=mysql://root:sua_senha@localhost:3306/alpha_bank

JWT_SECRET=mude-este-secret-em-producao-use-algo-aleatorio
JWT_EXPIRATION=86400

CORS_ORIGIN=http://localhost:3000

RUST_LOG=info
```

### 4. Compilação e Execução

Para iniciar o servidor em modo de desenvolvimento:

```bash
cargo run
```

Para compilar e executar uma versão otimizada para produção:

```bash
cargo build --release
./target/release/alpha-bank-backend
```

## 📡 Testando a API

O servidor estará rodando em `http://localhost:8080`.

### Exemplo de Requisição (Registro)

**Usando cURL:**

```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "full_name": "João Silva",
    "email": "joao@example.com",
    "password": "senha123",
    "cpf": "123.456.789-00",
    "birth_date": "1990-01-15",
    "phone": "(11) 98765-4321"
  }'
```

### Ferramentas de Teste

*   **Postman/Insomnia:** Importe a collection de endpoints para facilitar os testes.
*   **PowerShell (Windows):** Utilize `Invoke-WebRequest` para realizar requisições.

## 📊 Endpoints Principais

| Módulo | Endpoint | Método | Descrição |
| :--- | :--- | :--- | :--- |
| **Público** | `/api/auth/register` | `POST` | Registrar usuário |
| | `/api/auth/login` | `POST` | Login |
| **Protegido** | `/api/me` | `GET` | Perfil do usuário |
| | `/api/transactions` | `GET` | Listar transações |
| | `/api/transactions` | `POST` | Criar transação |
| | `/api/transactions/{id}` | `PUT` | Atualizar transação |
| | `/api/transactions/{id}` | `DELETE` | Deletar transação |
| | `/api/categories` | `GET` | Listar categorias |
| | `/api/categories` | `POST` | Criar categoria |
| | `/api/categories/{id}` | `DELETE` | Deletar categoria |

## 🐛 Solução de Problemas (Troubleshooting)

| Erro | Causa Comum | Solução |
| :--- | :--- | :--- |
| `"DATABASE_URL must be set"` | Arquivo `.env` ausente ou mal configurado. | Certifique-se de que o arquivo `.env` está no diretório raiz e contém a variável. |
| `"Access denied for user"` | Credenciais de banco de dados incorretas. | Verifique o usuário e a senha na variável `DATABASE_URL`. |
| `"Can't connect to MySQL server"` | MySQL não está em execução ou porta bloqueada. | Inicie o serviço MySQL (via `services.msc` ou XAMPP). |
| `"Unknown database 'alpha_bank'"` | Banco de dados não criado. | Execute o script `schema.sql` no MySQL. |
| `Porta 8080 já em uso` | Outro serviço está usando a porta. | Altere a porta na variável `PORT` do arquivo `.env`. |

## 📦 Estrutura do Projeto

```
alpha-bank-backend-mysql/
├── src/
│   ├── db/mod.rs              # Conexão MySQL
│   ├── handlers/              # Controladores (Auth, Transactions, Categories, etc.)
│   ├── middleware/auth.rs     # JWT middleware
│   ├── models/mod.rs          # Estruturas de dados
│   ├── utils/mod.rs           # Utilitários
│   └── main.rs                # Servidor HTTP principal
├── schema.sql                 # Schema MySQL
├── Cargo.toml                 # Dependências
├── .env.example
└── README.md
```

## 🔐 Segurança

A API foi desenvolvida com foco em segurança:

*   Senhas hasheadas com Bcrypt.
*   Autenticação via JWT.
*   Validação de CPF.
*   Proteção contra SQL Injection (garantida pelo SQLx).
*   CORS configurado para desenvolvimento.

## 💡 Dicas de Desenvolvimento

*   Utilize **Git Bash** ou **WSL** para comandos Unix-like no Windows.
*   O **Visual Studio Code** com a extensão `rust-analyzer` é altamente recomendado.
*   Instale `cargo-watch` (`cargo install cargo-watch`) para recarregamento automático do servidor durante o desenvolvimento: `cargo watch -x run`.

---

**Alpha Bank API** - Desenvolvido com Rust.
