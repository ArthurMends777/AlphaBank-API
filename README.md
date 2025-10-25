# 🏦 Alpha Bank - Backend API (MySQL + Windows)

API REST completa em **Rust** usando **Actix-web** e **MySQL** para o sistema de controle financeiro Alpha Bank.

**Configurado para desenvolvimento em Windows com MySQL Workbench.**

## 🚀 Tecnologias

- **Rust 1.70+**
- **Actix-web 4.4** - Framework web assíncrono
- **SQLx 0.7** - Driver MySQL com compile-time verification
- **MySQL 8.0+** - Banco de dados relacional
- **JWT** - Autenticação via JSON Web Tokens
- **Bcrypt** - Hash seguro de senhas

## 🔧 Configuração no Windows

### 1. Instalar Rust

Baixe e execute o instalador: https://rustup.rs/

```powershell
# Após instalação, reinicie o terminal e verifique:
rustc --version
cargo --version
```

### 2. Instalar MySQL

**Opção 1: MySQL Installer (Recomendado)**
1. Baixe: https://dev.mysql.com/downloads/installer/
2. Escolha "MySQL Installer for Windows"
3. Instale:
   - MySQL Server 8.0
   - MySQL Workbench
   - MySQL Shell (opcional)
4. Durante instalação, defina senha do root

**Opção 2: XAMPP**
1. Baixe: https://www.apachefriends.org/
2. Instale e inicie o MySQL pelo painel

### 3. Configurar MySQL

**Abra MySQL Workbench:**

1. Conecte ao servidor local (root)
2. Execute o arquivo `schema.sql`:
   - File → Open SQL Script → Selecione `schema.sql`
   - Clique no raio ⚡ (Execute)

Ou via MySQL Shell/Command Line:

```sql
mysql -u root -p < schema.sql
```

### 4. Configurar Variáveis de Ambiente

Copie o arquivo de exemplo:

```powershell
copy .env.example .env
```

Edite `.env` com suas configurações:

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

### 5. Compilar e Executar

```powershell
# Desenvolvimento
cargo run

# Produção (otimizado)
cargo build --release
.\target\release\alpha-bank-backend.exe
```

**Primeira execução demora** (compila todas as dependências).

## 📡 Testando a API

### Usando PowerShell

```powershell
# Health check
Invoke-WebRequest -Uri http://localhost:8080/health

# Registrar usuário
$body = @{
    full_name = "João Silva"
    email = "joao@example.com"
    password = "senha123"
    cpf = "123.456.789-00"
    birth_date = "1990-01-15"
    phone = "(11) 98765-4321"
} | ConvertTo-Json

Invoke-WebRequest -Uri http://localhost:8080/api/auth/register `
    -Method POST `
    -ContentType "application/json" `
    -Body $body
```

### Usando cURL (Git Bash ou WSL)

```bash
# Health check
curl http://localhost:8080/health

# Registrar
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

### Usando Postman/Insomnia

1. Importe a collection (criar arquivo JSON com endpoints)
2. Configure base URL: `http://localhost:8080/api`
3. Teste os endpoints

## 📊 Endpoints Disponíveis

### Autenticação (Públicos)

```
POST /api/auth/register  - Registrar usuário
POST /api/auth/login     - Login
```

### Protegidos (Requerem JWT)

```
GET    /api/me                    - Perfil do usuário
GET    /api/transactions          - Listar transações
POST   /api/transactions          - Criar transação
GET    /api/transactions/{id}     - Buscar transação
PUT    /api/transactions/{id}     - Atualizar transação
DELETE /api/transactions/{id}     - Deletar transação
GET    /api/categories            - Listar categorias
POST   /api/categories            - Criar categoria
DELETE /api/categories/{id}       - Deletar categoria
```

## 🗄️ Verificando o Banco de Dados

### MySQL Workbench

1. Conecte ao servidor
2. Selecione database: `USE alpha_bank;`
3. Visualize tabelas:

```sql
SHOW TABLES;

SELECT * FROM users;
SELECT * FROM categories;
SELECT * FROM transactions;
```

### Via Código Rust

Adicione logs no código:

```rust
println!("User created: {:?}", user);
```

## 🐛 Troubleshooting

### Erro: "DATABASE_URL must be set"

**Solução:** Certifique-se que o arquivo `.env` existe e está no diretório raiz do projeto.

### Erro: "Access denied for user"

**Solução:** Verifique usuário e senha no `DATABASE_URL`:

```env
DATABASE_URL=mysql://root:SUA_SENHA_AQUI@localhost:3306/alpha_bank
```

### Erro: "Can't connect to MySQL server"

**Causas:**
- MySQL não está rodando
- Porta 3306 bloqueada

**Solução:**
- Abra "Services" (Win+R → `services.msc`)
- Procure "MySQL80" e inicie o serviço
- Ou inicie pelo XAMPP Control Panel

### Erro: "Unknown database 'alpha_bank'"

**Solução:** Execute o `schema.sql` no MySQL Workbench.

### Porta 8080 já em uso

**Solução:** Altere a porta no `.env`:

```env
PORT=8081
```

## 🔥 Hot Reload (Opcional)

Instale `cargo-watch` para recarregar automaticamente:

```powershell
cargo install cargo-watch
cargo watch -x run
```

## 📦 Estrutura do Projeto

```
alpha-bank-backend-mysql/
├── src/
│   ├── db/mod.rs              # Conexão MySQL
│   ├── handlers/              # Controladores
│   │   ├── auth.rs
│   │   ├── transactions.rs
│   │   └── categories.rs
│   ├── middleware/auth.rs     # JWT middleware
│   ├── models/mod.rs          # Estruturas de dados
│   ├── utils/mod.rs           # Utilitários
│   └── main.rs                # Servidor HTTP
├── schema.sql                 # Schema MySQL
├── Cargo.toml                 # Dependências
├── .env.example
└── README.md
```

## 🔐 Segurança

✅ Senhas hasheadas com Bcrypt  
✅ Autenticação JWT  
✅ Validação de CPF  
✅ Proteção SQL Injection (SQLx)  
✅ CORS configurado  

## 🚀 Deploy (Produção)

### Compilar para Windows

```powershell
cargo build --release
```

Executável estará em: `target\release\alpha-bank-backend.exe`

### Configurar Serviço Windows (Opcional)

Use **NSSM** (Non-Sucking Service Manager):

1. Baixe: https://nssm.cc/download
2. Instale o serviço:

```powershell
nssm install AlphaBankAPI "C:\caminho\alpha-bank-backend.exe"
nssm set AlphaBankAPI AppDirectory "C:\caminho"
nssm start AlphaBankAPI
```

## 📝 Próximos Passos

- [ ] Implementar Goals endpoints
- [ ] Implementar Recurring endpoints
- [ ] Implementar Notifications endpoints
- [ ] Adicionar testes
- [ ] Documentação Swagger

## 💡 Dicas

- Use **Git Bash** ou **WSL** para comandos Unix
- **Visual Studio Code** é recomendado (extensão rust-analyzer)
- Mantenha o MySQL rodando durante desenvolvimento
- Use **Postman** para testar endpoints

---

**Alpha Bank API** - Desenvolvido com 🦀 Rust para Windows

