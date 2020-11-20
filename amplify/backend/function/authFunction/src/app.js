/*
Copyright 2017 - 2017 Amazon.com, Inc. or its affiliates. All Rights Reserved.
Licensed under the Apache License, Version 2.0 (the "License"). You may not use this file except in compliance with the License. A copy of the License is located at
    http://aws.amazon.com/apache2.0/
or in the "license" file accompanying this file. This file is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and limitations under the License.
*/

/* Amplify Params - DO NOT EDIT
	ENV
	REGION
Amplify Params - DO NOT EDIT */

var express = require('express')
var bodyParser = require('body-parser')
var awsServerlessExpressMiddleware = require('aws-serverless-express/middleware')
const randomstring = require('randomstring')
const { getScript } = require('./callback-script')
const { getOauth } = require('./oauth')

const loginAuthTarget = process.env.AUTH_TARGET || '_self'
const oauthProvider = process.env.OAUTH_PROVIDER || 'github'

// declare a new express app
var app = express()
app.use(bodyParser.json())
app.use(awsServerlessExpressMiddleware.eventContext())

// Enable CORS for all methods
app.use(function (req, res, next) {
  res.header('Access-Control-Allow-Origin', '*')
  res.header(
    'Access-Control-Allow-Headers',
    'Origin, X-Requested-With, Content-Type, Accept'
  )
  next()
})

app.get('/auth', function (req, res) {
  res.redirect('/master/auth/authorize')
})

app.get('/auth/authorize', async function (req, res) {
  // Authorization uri definition
  const oauth2 = await getOauth()
  const authorizationUri = oauth2.authorizeURL({
    redirect_uri: process.env.REDIRECT_URL,
    scope: process.env.SCOPES || 'repo,user',
    state: randomstring.generate(32),
  })
  res.redirect(authorizationUri)
})

app.get('/auth/callback', async function (req, res) {
  const code = req.query.code
  var options = {
    code: code,
  }
  const oauth2 = await getOauth()

  let mess, content
  try {
    const accessToken = await oauth2.getToken(options)
    const token = oauth2.createToken(result)
    mess = 'success'
    content = {
      token: token.token.access_token,
      provider: oauthProvider,
    }
  } catch (error) {
    console.error('Access Token Error', error.message)
    mess = 'error'
    content = JSON.stringify(error)
  }
  return res.send(getScript(mess, content))
})

app.get('/auth/success', function (req, res) {
  res.status(204).send('')
})

app.listen(3000, function () {
  console.log('App started')
})

// Export the app object. When executing the application local this does nothing. However,
// to port it to AWS Lambda we will create a wrapper around that will load the app from
// this file
module.exports = app
