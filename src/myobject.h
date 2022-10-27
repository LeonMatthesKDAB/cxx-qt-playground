#pragma once

#include "rust/cxx.h"

#include <QtCore/QObject>

#include <memory>

class MyObjectRust;

class Opaque {
    int mySecretThing;
};

class MyObject : public QObject
{
    Q_OBJECT

    Q_PROPERTY(int prop READ getProp WRITE setProp NOTIFY propChanged)

public:
    MyObject(QObject* parent = nullptr);

    const MyObjectRust& unsafe_rust() const;
    MyObjectRust& unsafe_rust_mut();

    Q_INVOKABLE void add();

    int getProp() const;
    void setProp(int value);

Q_SIGNALS:
    void propChanged();

private:
    

    // friend const MyObjectRust* cxxbridge1$MyObject$rust(const MyObject&) noexcept;
    // friend MyObjectRust* cxxbridge1$MyObject$rust_mut(MyObject&) noexcept;
    rust::Box<MyObjectRust> m_rust;
};
