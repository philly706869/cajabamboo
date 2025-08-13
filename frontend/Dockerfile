FROM node:22-trixie AS builder
COPY . /app
WORKDIR /app
RUN npm i && npm run build

FROM node:22-alpine
COPY --from=builder /app/package.json /app/package.json
COPY --from=builder /app/package-lock.json /app/package-lock.json
COPY --from=builder /app/build /app/build
WORKDIR /app
RUN npm i --omit dev
CMD ["node", "build"]
