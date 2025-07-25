import test from 'ava'
import { setPassword, getPassword, deletePassword } from "../index.js"

test("should fail trying to delete a password that doesn't exist", t => {
    try {
        deletePassword("test-service", "test-account")
        t.fail()
    } catch (error) {
        t.pass()
    }
})

test("should find no password", t => {
    const password = getPassword("test-service", "test-account")
    t.is(password, null)
})

test("should set a password", t => {
    setPassword("test-service", "test-account", "test-password")
    t.pass()
})

test("should get a password", t => {
    const password = getPassword("test-service", "test-account")
    t.is(password, "test-password")
})

test("should delete a password", t => {
    deletePassword("test-service", "test-account")
    t.pass()
})

test("should find no password after deletion", t => {
    const password = getPassword("test-service", "test-account")
    t.is(password, null)
})
