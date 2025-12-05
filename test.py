# this package
from notify_rs import TIMEOUT_DEFAULT, URGENCY_CRITICAL, Notification, NotificationHandle

n = Notification().summary("The summary").body("The body").urgency(URGENCY_CRITICAL)

n.icon("firefox")
assert n.get_summary() == "The summary"
assert n.get_body() == "The body"
assert n.get_subtitle() is None
# No method for this; it's in hints, which isn't implemented
# assert n.get_urgency() == URGENCY_CRITICAL
assert n.get_timeout() == TIMEOUT_DEFAULT

assert isinstance(n.show(), NotificationHandle)
