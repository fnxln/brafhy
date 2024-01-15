FROM rust:latest

# Defina o diretório de trabalho
WORKDIR /app/back

COPY Cargo.toml Cargo.lock ./

# Copie o código fonte e outros arquivos necessários
COPY . .
# Adicione qualquer outro arquivo que possa ser necessário aqui

# Instale o sqlx-cli
RUN cargo install sqlx-cli

# Construa o projeto
RUN cargo build --release

# Copie o entrypoint script e dê permissões de execução
COPY entrypoint.sh .
RUN chmod +x entrypoint.sh

# Exponha a porta em que seu servidor estará ouvindo
EXPOSE 3000

# Use entrypoint.sh para inicializar e depois executar o comando principal
ENTRYPOINT ["./entrypoint.sh"]
CMD ["./target/release/brafhy"]
