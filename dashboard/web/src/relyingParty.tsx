import { Button, TextField } from "@charcoal-ui/react";
import React, { Suspense, useEffect, useRef, useState } from "react";
import { Modal, ModalBody, ModalHeader } from "./modal";
import { relyingParty } from "./types";
import { useSuspendApi, fetchApi } from "./useApi";
import { useToken } from "./useToken";

export default function RelyingParty() {
  const { data: relyingPartiesResponse } = useSuspendApi(
    useToken(),
    "/dash/rp/list",
    {}
  );
  const relyingParties = relyingPartiesResponse.values;

  const updateView = () => {
    location.reload();
  };

  const createModalOpen = useState(false);

  return (
    <>
      <div>
        <CreateRPModal
          open={createModalOpen}
          updateView={updateView}
        ></CreateRPModal>
        <div className="p-8 inline-block">
          <Button size="S" variant="Navigation" onClick={() => updateView()}>
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
        </div>
        {relyingParties
          .sort((a, b) => (a.client_id < b.client_id ? -1 : 1))
          .map((rp) => (
            <RelyingPartyListItem
              key={rp.client_id}
              rp={rp}
              updateView={updateView}
            />
          ))}
      </div>
    </>
  );
}

const RelyingPartyListItem = ({
  rp,
  updateView,
}: {
  rp: relyingParty;
  updateView: () => void;
}) => {
  const deleteModalOpen = useState(false);
  const editModalOpen = useState(false);
  const updateSecretModalOpen = useState(false);
  const setRoleAccessModalOpen = useState(false);

  const rolesRes = useSuspendApi(
    useToken(),
    "/dash/rp/role/list",
    { client_id: rp.client_id },
    "/dash/rp/role/list" + rp.client_id
  );

  return (
    <div key={rp.client_id} className="p-8 mb-16 bg-surface3">
      <DeleteRPModal
        open={deleteModalOpen}
        clientId={rp.client_id}
        updateView={updateView}
      />
      <EditModal open={editModalOpen} rp={rp} updateView={updateView} />
      <NewSecretModal open={updateSecretModalOpen} client_id={rp.client_id} />
      <SetRoleAccessModal
        clientId={rp.client_id}
        open={setRoleAccessModalOpen}
        updateView={updateView}
      />
      <h2 className="font-bold text-base">{rp.client_id}</h2>
      <div className="mb-8">ロール {rolesRes.data.roles.join(", ")}</div>
      <div>
        <div className="inline-block p-8 pl-0">
          <Button
            size="S"
            variant="Navigation"
            onClick={() => setRoleAccessModalOpen[1](true)}
          >
            roles
          </Button>
        </div>
        <div className="inline-block p-8 pl-0">
          <Button
            size="S"
            variant="Navigation"
            onClick={() => editModalOpen[1](true)}
          >
            redirect_uris
          </Button>
        </div>
        <div className="inline-block p-8 pl-0">
          <Button
            size="S"
            variant="Overlay"
            onClick={() => deleteModalOpen[1](true)}
          >
            DELETE
          </Button>
        </div>
        <div className="inline-block p-8 pl-0">
          <Button
            size="S"
            variant="Overlay"
            onClick={() => updateSecretModalOpen[1](true)}
          >
            client_secret
          </Button>
        </div>
        <h3 className="font-bold">Redirect URIs</h3>
        <ul>
          {rp.redirect_uris.map((uri: string) => (
            <li key={uri}>{uri}</li>
          ))}
        </ul>
      </div>
    </div>
  );
};

type createRPModalProps = {
  open: [boolean, React.Dispatch<React.SetStateAction<boolean>>];
  updateView: () => void;
};
const CreateRPModal = ({ open, updateView }: createRPModalProps) => {
  const [id, setId] = useState("");
  const onSubmit = () => {
    if (id) {
      fetchApi(useToken(), "/dash/rp/create", { client_id: id }).then((res) => {
        setClientSecret(res.client_secret);
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

  const [clientSecret, setClientSecret] = useState("");

  const onCloseClick = () => {
    setClientSecret("");
    updateView();
    open[1](false);
  };

  return (
    <Modal open={open} isDissmissable={false} onClose={onCloseClick}>
      <ModalHeader>Relying Party の作成</ModalHeader>
      <ModalBody>
        {!clientSecret && (
          <>
            <TextField
              label="client_id"
              showLabel
              required
              className="mb-24"
              onChange={onChange}
            ></TextField>
            <Button
              variant="Primary"
              fixed
              onClick={onSubmit}
              disabled={disabled}
            >
              作成する
            </Button>
          </>
        )}
        {clientSecret && (
          <>
            <TextField
              label="client_secret"
              showLabel
              className="mb-24"
              value={clientSecret}
            ></TextField>
            <Button variant="Primary" fixed onClick={onCloseClick}>
              閉じる
            </Button>
          </>
        )}
      </ModalBody>
    </Modal>
  );
};

type deleteRPModalProps = {
  open: [boolean, React.Dispatch<React.SetStateAction<boolean>>];
  clientId: string;
  updateView: () => void;
};
const DeleteRPModal = ({ open, clientId, updateView }: deleteRPModalProps) => {
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
    fetchApi(useToken(), "/dash/rp/delete", { client_id: clientId }).then(
      () => {
        updateView();
        open[1](false);
      }
    );
  };

  return (
    <Modal open={open}>
      <ModalHeader>Relying Party の削除</ModalHeader>
      <ModalBody>
        <div className="mb-24">
          <span className="font-bold">{clientId}</span> を削除しますか？
        </div>
        <Button variant="Danger" fixed disabled={disabled} onClick={onSubmit}>
          削除する
        </Button>
      </ModalBody>
    </Modal>
  );
};

type newSecretModalProps = {
  open: [boolean, React.Dispatch<React.SetStateAction<boolean>>];
  client_id: string;
};
const NewSecretModal = ({ open, client_id }: newSecretModalProps) => {
  const [secret, setSecret] = useState("");
  const onClick = async () => {
    const res = await fetchApi(useToken(), "/dash/rp/update_secret", {
      client_id,
    });
    setSecret(res.client_secret);
  };
  useEffect(() => {
    if (!open[0]) {
      setSecret("");
    }
  }, [open[0]]);
  return (
    <Modal open={open} isDissmissable={false}>
      <ModalHeader>client_secret の更新</ModalHeader>
      <ModalBody>
        <div className="mb-8">
          <Button variant="Danger" fixed onClick={onClick}>
            更新する
          </Button>
        </div>
        <TextField label="client_secret" showLabel value={secret} />
      </ModalBody>
    </Modal>
  );
};

type editModalProps = {
  open: [boolean, React.Dispatch<React.SetStateAction<boolean>>];
  rp: any;
  updateView: () => void;
};
const EditModal = ({ open, rp, updateView }: editModalProps) => {
  const [uris, setUris] = useState<string[]>([...rp.redirect_uris]);
  useEffect(() => {
    setUris([...rp.redirect_uris]);
  }, [open[0]]);
  const onChange = (v: string) => {
    setUris(v.split("\n"));
  };
  const onSubmit = () => {
    const deletes = [
      ...rp.redirect_uris.map((uri: string) =>
        fetchApi(useToken(), "/dash/rp/redirect_uri/remove", {
          client_id: rp.client_id,
          redirect_uri: uri,
        })
      ),
    ];
    const adds = [
      ...uris
        .filter((v) => v.trim() !== "")
        .map((uri: string) =>
          fetchApi(useToken(), "/dash/rp/redirect_uri/add", {
            client_id: rp.client_id,
            redirect_uri: uri,
          })
        ),
    ];

    Promise.allSettled(deletes)
      .then(() => Promise.allSettled(adds))
      .then(() => {
        updateView();
        open[1](false);
      });
  };

  return (
    <Modal open={open} isDissmissable={false}>
      <ModalHeader>Redirect URI の編集</ModalHeader>
      <ModalBody>
        <TextField
          multiline
          label="redirect_uris"
          placeholder="redirect_uris"
          value={uris.join("\n")}
          onChange={onChange}
          className="mb-24"
        ></TextField>
        <Button variant="Primary" fixed onClick={onSubmit}>
          確定する
        </Button>
      </ModalBody>
    </Modal>
  );
};

type setSetRoleAccessProps = {
  open: [boolean, React.Dispatch<React.SetStateAction<boolean>>];
  clientId: string;
  updateView: () => void;
};
function SetRoleAccessModal({
  open,
  clientId,
  updateView,
}: setSetRoleAccessProps) {
  const rolesRes = useSuspendApi(
    useToken(),
    "/dash/rp/role/list",
    {
      client_id: clientId,
    },
    "/dash/rp/role/list/" + clientId
  );

  const [roles, setRoles] = useState(rolesRes.data.roles);

  const onSubmit = async () => {
    await fetchApi(useToken(), "/dash/rp/role/set", {
      client_id: clientId,
      roles,
    });
    updateView();
    open[1](false);
  };

  return (
    <Modal open={open} isDissmissable={false}>
      <ModalHeader>アクセス許可</ModalHeader>
      <ModalBody>
        <div className="mb-24">
          <span className="font-bold">{clientId}</span>{" "}
          へのログインを許可するロール
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
