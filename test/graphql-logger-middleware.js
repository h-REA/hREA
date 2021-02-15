/**
 * Helper for injecting error logging code into resolver callbacks to give better
 * error messages in the console.
 *
 * @package: HoloREA
 * @since:   2019-12-02
 */

const deepForEach = require('deep-for-each')
const isFunction = require('is-function')

const injectLoggingCode = (options = {}) => (resolvers) => {
  const prefix = options.prefix || ''
  const logger = options.logger || console.log.bind(console)
  const errorLogger = options.errorLogger || console.warn.bind(console, '\x1b[1m\x1b[31mGraphQL query error\x1b[0m')

  // Deeply iterate over all resolvers
  deepForEach(resolvers, (value, prop, subject, path) => {
    // If we have a function
    if (isFunction(value)) {
      // Construct the string to be logged
      const string = prefix + path
      // Replace the original value with a wrapper function
      subject[prop] = async function wrapped (...args) {
        logger('[\x1b[1mGraphQL\x1b[0m]: ', string)

        try {
          const res = await value(...args)

          if (res instanceof Error) {
            errorLogger(res)
          } else {
            logger('[\x1b[1mresult\x1b[0m]: ', res)
          }

          return res
        } catch (err) {
          errorLogger(err)

          throw err // re-throw so that it gets included in the result `errors` payload
        }
      }
    }
  })

  return resolvers
}

module.exports = injectLoggingCode
