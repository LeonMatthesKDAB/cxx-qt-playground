#include "myobject.h"

#include "lib.rs.h"

MyObject::MyObject(QObject* parent)
    : QObject(parent)
    , m_rust(create_rs())
{

}

void MyObject::add()
{
    // mutex

    // namespaced, myobject::internals::add(this)
    ::add(*this);
}

const MyObjectRust& MyObject::unsafe_rust() const
{
    return *m_rust;
}

MyObjectRust& MyObject::unsafe_rust_mut()
{
    return *m_rust;
}

int MyObject::getProp() const
{
    return m_rust->get_prop();
}

void MyObject::setProp(int value)
{
    set_prop(*this, value);
}
