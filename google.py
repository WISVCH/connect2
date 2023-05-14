from google.auth.transport import requests
from google.oauth2 import service_account

# https://www.googleapis.com/auth/sqlservice.login https://www.googleapis.com/auth/cloud-platform https://www.googleapis.com/auth/compute https://www.googleapis.com/auth/appengine.admin https://www.googleapis.com/auth/userinfo.email openid
CREDENTIAL_SCOPES = ["https://www.googleapis.com/auth/cloud-identity.groups.readonly"]

CREDENTIALS_KEY_PATH = 'key.json'

def get_service_account_token():
  credentials = service_account.Credentials.from_service_account_file(
          CREDENTIALS_KEY_PATH, scopes=CREDENTIAL_SCOPES)
  #   Set sub
  credentials = credentials.with_subject('root@ch.tudelft.nl')
  credentials.refresh(requests.Request())
  return credentials.token

if __name__ == '__main__':
  print(get_service_account_token())