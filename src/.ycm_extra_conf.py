def Settings( **kwargs ):
  if kwargs[ 'language' ] == 'rust':
    return {
      'ls': {
          'diagnostics': {
              'enable': True,
              'disabled': ["unresolved-proc-macro", "unresolved-macro-call", "unresolved-import"]
          },
          'updates': {
              'channel': 'nightly'
          },
          'server': {
              'extraEnv': {
                  'DATABASE_URL': 'mysql://root:password@localhost/database'
                  }
          }
        }
      }
