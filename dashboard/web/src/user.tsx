import { Button, TextField } from "@charcoal-ui/react";
import React, { Suspense, useEffect, useRef, useState } from "react";
import { Modal, ModalBody, ModalHeader } from "./modal";
import { fetchApi, useSuspendApi } from "./useApi";
import { useToken } from "./useToken";

type user = {
  user_id: string;
  has_password: boolean;
};

export default function User() {
  const { data: usersResponse, mutate } = useSuspendApi(
    useToken(),
    "/dash/user/list",
    {}
  );
  const users = usersResponse.values;

  const createModalOpen = useState(false);

  const updateView = () => {
    location.reload();
  };

  return (
    <>
      <div>
        <div className="p-8 inline-block">
          <Button size="S" variant="Navigation" onClick={updateView}>
            RELOAD
          </Button>
        </div>
        <div className="p-8 inline-block mb-24">
          <Button
            size="S"
            variant="Primary"
            onClick={() => createModalOpen[1](true)}
          >
            CREATE
          </Button>
          <CreateUserModal open={createModalOpen} updateView={updateView} />
        </div>
        {users
          .sort((a, b) => (a.user_id < b.user_id ? -1 : 1))
          .map((user) => (
            <UserListItem
              key={user.user_id}
              user={user}
              updateView={updateView}
            />
          ))}
      </div>
    </>
  );
}

const UserListItem = ({
  user,
  updateView,
}: {
  user: user;
  updateView: () => void;
}) => {
  const deleteModalOpen = useState(false);

  const passwordEditOpen = useState(false);

  const logModalOpen = useState(false);

  const setRolesModalOpen = useState(false);

  const rolesResponse = useSuspendApi(
    useToken(),
    "/dash/user/role/list",
    {
      user_id: user.user_id,
    },
    "/dash/user/role/list/" + user.user_id
  );

  const userinfoResonse = useSuspendApi(
    useToken(),
    "/dash/user/userinfo/get",
    {
      user_id: user.user_id,
    },
    "/dash/user/userinfo/get/" + user.user_id
  );

  const [isUserinfoOpen, setIsUserinfoOpen] = useState(false);

  return (
    <div key={user.user_id} className="p-8 mb-16 bg-surface3">
      <h2 className="font-bold text-base mb-8">{user.user_id}</h2>
      <div className="mb-8">
        パスワード {user.has_password ? "設定済み" : "未設定"}
      </div>
      <div className="mb-8">ロール {rolesResponse.data.roles.join(", ")}</div>
      <details
        className="mb-8"
        onToggle={(e) => {
          setIsUserinfoOpen(e.currentTarget.open);
        }}
      >
        {userinfoResonse.data.value}
        <summary className="whitespace-nowrap text-ellipsis overflow-hidden">
          userinfo {!isUserinfoOpen && userinfoResonse.data.value}
        </summary>
      </details>
      <div className="mb-8">
        <div className="inline-block p-8 pl-0">
          <Button
            size="S"
            variant="Navigation"
            onClick={() => passwordEditOpen[1](true)}
          >
            PASSWORD
          </Button>
          <SetPasswordModal
            updateView={updateView}
            open={passwordEditOpen}
            userId={user.user_id}
          />
        </div>
        <div className="inline-block p-8 pl-0">
          <Button
            size="S"
            variant="Navigation"
            onClick={() => setRolesModalOpen[1](true)}
          >
            ROLE
          </Button>
          <SetUserRoleModal
            updateView={updateView}
            open={setRolesModalOpen}
            userId={user.user_id}
          />
        </div>
        <div className="inline-block p-8 pl-0">
          <Button
            size="S"
            variant="Overlay"
            onClick={() => deleteModalOpen[1](true)}
          >
            DELETE
          </Button>
          <DeleteUserModal
            open={deleteModalOpen}
            userId={user.user_id}
            updateView={updateView}
          />
        </div>
      </div>
      <div>
        <Button
          size="S"
          variant="Navigation"
          onClick={() => logModalOpen[1](true)}
        >
          ログイン履歴
        </Button>
        <LogModal userId={user.user_id} open={logModalOpen} />
      </div>
    </div>
  );
};

type logModalProps = {
  open: [boolean, React.Dispatch<React.SetStateAction<boolean>>];
  userId: string;
};
const LogModal = ({ open, userId }: logModalProps) => {
  return (
    <Modal open={open}>
      <ModalHeader>ログイン履歴</ModalHeader>
      <ModalBody>
        <Suspense fallback={<>Loading</>}>
          <Logs userId={userId} />
        </Suspense>
      </ModalBody>
    </Modal>
  );
};

const Logs = ({ userId }: { userId: string }) => {
  const { data, mutate } = useSuspendApi(
    useToken(),
    "/dash/user/authentication/list",
    { user_id: userId },
    "/dash/user/authentication/list/" + userId
  );
  useEffect(() => {
    mutate();
  }, []);
  return (
    <ul>
      {data.values.map((log) => (
        <li key={log.iat}>
          {new Date(log.iat * 1000).toLocaleString()} {`${log.remote_addr}`}:{" "}
          {log.aud}
        </li>
      ))}
    </ul>
  );
};

type createUserModalProps = {
  open: [boolean, React.Dispatch<React.SetStateAction<boolean>>];
  updateView: () => void;
};
const CreateUserModal = ({ open, updateView }: createUserModalProps) => {
  const [id, setId] = useState("");
  const onSubmit = () => {
    if (id) {
      fetchApi(useToken(), "/dash/user/create", { user_id: id }).then(() => {
        open[1](false);
        updateView();
      });
    }
  };
  const [disabled, setDisabled] = useState(true);
  const onChange = (v: string) => {
    setId(v);
    if (v) {
      setDisabled(false);
    } else {
      setDisabled(true);
    }
  };

  return (
    <Modal open={open} isDissmissable={false}>
      <ModalHeader>ユーザーの作成</ModalHeader>
      <ModalBody>
        <TextField
          label="user_id"
          showLabel
          required
          className="mb-24"
          onChange={onChange}
        ></TextField>
        <Button variant="Primary" fixed onClick={onSubmit} disabled={disabled}>
          作成する
        </Button>
      </ModalBody>
    </Modal>
  );
};

type deleteUserModalProps = {
  open: [boolean, React.Dispatch<React.SetStateAction<boolean>>];
  userId: string;
  updateView: () => void;
};
const DeleteUserModal = ({
  open,
  userId,
  updateView,
}: deleteUserModalProps) => {
  const [disabled, setDisabled] = useState(true);
  useEffect(() => {
    if (open[0]) {
      setDisabled(true);
      setTimeout(() => {
        setDisabled(false);
      }, 2000);
    }
  }, [open[0]]);

  const onSubmit = () => {
    fetchApi(useToken(), "/dash/user/delete", { user_id: userId }).then(() => {
      updateView();
      open[1](false);
    });
  };

  return (
    <Modal open={open}>
      <ModalHeader>ユーザーの削除</ModalHeader>
      <ModalBody>
        <div className="mb-24">
          <span className="font-bold">{userId}</span> を削除しますか？
        </div>
        <Button variant="Danger" fixed disabled={disabled} onClick={onSubmit}>
          削除する
        </Button>
      </ModalBody>
    </Modal>
  );
};

type setPasswordModalProps = {
  open: [boolean, React.Dispatch<React.SetStateAction<boolean>>];
  userId: string;
  updateView: () => void;
};
const SetPasswordModal = ({
  userId,
  open,
  updateView,
}: setPasswordModalProps) => {
  const [password, setPassword] = useState("");

  useEffect(() => {
    if (open[0]) {
      setPassword("");
    }
  }, [open[0]]);

  const onSubmit = () => {
    if (password == "") {
      fetchApi(useToken(), "/dash/user/password/remove", {
        user_id: userId,
      }).then(() => {
        updateView();
        open[1](false);
      });
    } else {
      fetchApi(useToken(), "/dash/user/password/change", {
        user_id: userId,
        password,
      }).then(() => {
        updateView();
        open[1](false);
      });
    }
  };

  return (
    <Modal open={open}>
      <ModalHeader>パスワードの設定</ModalHeader>
      <ModalBody>
        <div className="mb-24">
          <span className="font-bold">{userId}</span> のパスワードを変更
        </div>
        <TextField
          label="パスワード"
          placeholder="パスワード"
          className="mb-24"
          onChange={(v) => setPassword(v)}
        ></TextField>
        <Button variant="Primary" fixed onClick={onSubmit}>
          変更する
        </Button>
      </ModalBody>
    </Modal>
  );
};

type setUserRoleModalProps = {
  open: [boolean, React.Dispatch<React.SetStateAction<boolean>>];
  userId: string;
  updateView: () => void;
};
function SetUserRoleModal({ open, userId, updateView }: setUserRoleModalProps) {
  const rolesResponse = useSuspendApi(
    useToken(),
    "/dash/user/role/list",
    {
      user_id: userId,
    },
    "/dash/user/role/list/" + userId
  );
  const initialRoles = rolesResponse.data.roles;

  const [roles, setRoles] = useState<string[]>(initialRoles);

  const onSubmit = async () => {
    await fetchApi(useToken(), "/dash/user/role/set", {
      user_id: userId,
      roles,
    });
    updateView();
    open[1](false);
  };

  return (
    <Modal open={open} isDissmissable={false}>
      <ModalHeader>ロールの設定</ModalHeader>
      <ModalBody>
        <div className="mb-24">
          <span className="font-bold">{userId}</span> のロールを変更
        </div>
        <TextField
          label="ロール"
          placeholder="ロール"
          multiline
          className="mb-24"
          value={roles.join("\n")}
          onChange={(v) => {
            setRoles(v.split(/\s+/));
          }}
        ></TextField>
        <Button variant="Primary" fixed onClick={onSubmit}>
          変更する
        </Button>
      </ModalBody>
    </Modal>
  );
}
