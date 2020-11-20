const { SecretsManager } = require('aws-sdk')
const simpleOauthModule = require('simple-oauth2')

const secretsManager = new SecretsManager()

exports.getOauth = async function getOauth() {
  const secrets = await secretsManager
    .getSecretValue({ SecretId: 'michalvankodev_github_oauth' })
    .promise()
    .then((data) => {
      return JSON.parse(data.SecretString)
    })

  const config = {
    OAUTH_CLIENT_ID: secrets.CLIENT_ID,
    OAUTH_CLIENT_SECRET: secrets.CLIENT_SECRETS,
    GIT_HOSTNAME: 'https://github.com',
    OAUTH_TOKEN_PATH: '/login/oauth/access_token',
    OAUTH_AUTHORIZE_PATH: '/login/oauth/authorize',
  }

  return new simpleOauthModule.AuthorizationCode({
    client: {
      id: config.OAUTH_CLIENT_ID,
      secret: config.OAUTH_CLIENT_SECRET,
    },
    auth: {
      tokenHost: config.GIT_HOSTNAME,
      tokenPath: config.OAUTH_TOKEN_PATH,
      authorizePath: config.OAUTH_AUTHORIZE_PATH,
    },
  })
}
